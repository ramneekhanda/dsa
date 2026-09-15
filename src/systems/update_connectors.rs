use crate::c_log;
use crate::components::message::Messages;
use crate::components::node::{NodeMarker, SelectedNodeMarker};
use crate::resources::graph_def::GraphDefinitionRes;
use crate::{
    components::node_connector::*,
    parser::graphv2::{ConnectorStyle, GraphAttrs},
};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use std::collections::{HashMap, HashSet};

/// Roughly how many points `walk_path` samples along a connector, regardless
/// of how long it actually is in world units - see that function's doc
/// comment for why this needs to be length-independent.
const WALK_TARGET_POINTS: f32 = 120.0;
/// Floor on the sample spacing, so a very short (near-zero-length) connector
/// doesn't get a degenerate/huge point count.
const WALK_MIN_INTERVAL: f32 = 0.5;

/// Samples points along a path, spaced so the *count* stays roughly constant
/// (`WALK_TARGET_POINTS`) regardless of the path's actual length - used to
/// place an in-flight message along a connector's curve. Called only when a
/// connector's `Path` is (re)built, with the result cached on
/// `NodeConnector::walk_cache`, rather than every frame in the message
/// animation system (see that field's doc comment for why).
///
/// The interval used to be a fixed `0.1` world units regardless of path
/// length - fine for the small, tightly-packed example graphs this was
/// written against (connectors tens of units long), but on a larger/more
/// spread-out graph (a several-hundred-node stress-test graph's nodes are
/// spread across a much larger area - see `node_system::spawn_spread_radius`)
/// connectors can be many hundreds of units long, so a fixed `0.1` spacing
/// was generating thousands of sample points per connector - a real,
/// measurable cost distinct from (and larger than) the allocation cost fixed
/// alongside it, since this ran on every retrace (frequent - pulse
/// animations touch many connectors' endpoints across frames). A message
/// only needs on the order of a hundred points along its path for smooth
/// animation regardless of the path's physical length, so scaling the
/// interval to the path's own length keeps both quality and cost constant
/// across graph scales.
pub fn walk_path(path: &lyon_algorithms::path::Path) -> Vec<[f32; 2]> {
    use lyon_algorithms::length::approximate_length;
    use lyon_algorithms::walk::{walk_along_path, RegularPattern, WalkerEvent};

    let tolerance = 0.5; // The path flattening tolerance.
    let length = approximate_length(path.iter(), tolerance);
    let interval = (length / WALK_TARGET_POINTS).max(WALK_MIN_INTERVAL);

    let mut x: Vec<[f32; 2]> = vec![];
    let mut pattern = RegularPattern {
        callback: &mut |event: WalkerEvent| {
            x.push(event.position.to_array());
            true // Return true to continue walking the path.
        },
        interval,
    };

    let start_offset = 0.0; // Start walking at the beginning of the path.

    walk_along_path(path.iter(), start_offset, tolerance, &mut pattern);
    x
}

/// Normalizes an (a, b) pair so `edge_key(a, b) == edge_key(b, a)` - a connector is
/// undirected and only needs to exist once regardless of which side declared the link.
/// Borrows rather than allocates - see the doc comment on `update_connectors`'s
/// `desired`/`existing` maps for why that matters here.
fn edge_key<'a>(a: &'a str, b: &'a str) -> (&'a str, &'a str) {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

/// Distance from a node's center to inset each connector endpoint by, so the
/// line visibly stops just outside the icon's edge instead of running into
/// or under it. Node icons are drawn at `ICON_WIDTH`/`ICON_HEIGHT` = 64px
/// (see `node_system.rs`) - half that, plus a small margin.
const NODE_RADIUS: f32 = 38.0;
/// How far the curve bows perpendicular to the A-B line, as a fraction of
/// the distance between the two node centers - clamped so very short links
/// don't degenerate and very long ones don't bow absurdly far.
const BOW_FRACTION: f32 = 0.12;
const BOW_MIN: f32 = 10.0;
const BOW_MAX: f32 = 48.0;

fn bow_amount(dist: f32) -> f32 {
    (dist * BOW_FRACTION).clamp(BOW_MIN, BOW_MAX)
}

/// Builds the connector path between node centers `a` and `b`, shaped
/// according to `graph_attrs.connector_style` (see `ConnectorStyle`'s doc
/// comment) - shared by both the initial spawn (`generate_line`) and the
/// per-frame retrace loop so newly-created and moving connectors can't
/// drift out of sync with each other.
///
/// `Curved` bows perpendicular to the A→B line (magnitude proportional to
/// distance, clamped) rather than always toward a fixed `(+50, +50)`
/// diagonal offset like before - that fixed offset could bow the "wrong"
/// way (e.g. for vertically stacked nodes) or look lopsided depending on
/// layout, since it didn't account for how the two nodes were actually
/// arranged. This way the arc direction and shape stay visually consistent
/// no matter how a graph is laid out or dragged.
///
/// `Straight` and `Step` are built from plain line segments rather than a
/// bezier. `Step` routes horizontal out from `a`, vertical to align, then
/// horizontal in to `b` (elbowed at the horizontal midpoint) - unless the
/// two nodes are already level, in which case it's just the one straight
/// segment. Both right-angle corners get rounded for free by the
/// connector's existing `LineJoin::Round` stroke (see `connector_stroke`),
/// no arc math needed.
fn build_connector_path(a: Vec2, b: Vec2, style: ConnectorStyle) -> Path {
    let delta = b - a;
    let dist = delta.length();
    let mut path_builder = PathBuilder::new();
    if dist < 1.0 {
        // Degenerate (nodes effectively on top of each other) - avoid a
        // divide-by-zero in the normalize below.
        path_builder.move_to(a);
        path_builder.line_to(b);
        return path_builder.build();
    }
    let dir = delta / dist;

    // Don't let the inset eat more than the two nodes' visual gap on very
    // short links (icons close together or briefly overlapping mid-drag).
    let inset = NODE_RADIUS.min(dist * 0.45);
    let start = a + dir * inset;
    let end = b - dir * inset;

    path_builder.move_to(start);
    match style {
        ConnectorStyle::Straight => {
            path_builder.line_to(end);
        }
        ConnectorStyle::Step => {
            // Horizontal out from `start`, vertical to align, horizontal in
            // to `end` - three segments, elbowed at the horizontal
            // midpoint. Skip the (degenerate, zero-length) vertical leg
            // when the two nodes are already level, rather than leaving a
            // redundant collinear vertex in the path.
            if (end.y - start.y).abs() < 0.5 {
                path_builder.line_to(end);
            } else {
                let mid_x = (start.x + end.x) / 2.0;
                path_builder.line_to(Vec2::new(mid_x, start.y));
                path_builder.line_to(Vec2::new(mid_x, end.y));
                path_builder.line_to(end);
            }
        }
        ConnectorStyle::Curved => {
            let perp = Vec2::new(-dir.y, dir.x);
            let bow = bow_amount(dist);
            let c1 = start + (end - start) * 0.33 + perp * bow;
            let c2 = start + (end - start) * 0.66 + perp * bow;
            path_builder.cubic_bezier_to(c1, c2, end);
        }
    }
    path_builder.build()
}

/// A connector's rounded-cap, rounded-join stroke - shared by both
/// `generate_line` and the retrace loop so newly-spawned and moving
/// connectors always look the same.
fn connector_stroke(color: Color) -> Stroke {
    Stroke {
        options: StrokeOptions::default()
            .with_line_width(BASE_WIDTH)
            .with_line_cap(LineCap::Round)
            .with_line_join(LineJoin::Round),
        color,
    }
}

/// Adds/removes connector entities to match the currently-declared `links`, and
/// retraces existing connectors' paths as their endpoints move. Runs as a
/// diff every frame (cheap at these graph sizes) rather than
/// despawning and rebuilding every connector whenever any single node is added or
/// removed - that used to happen on every `spawn()`/`despawn()` (and every YAML
/// edit), destroying every *other* connector's in-flight `Messages` queue along
/// with it.
///
/// Links are read from `node_instances` (the live, runtime-mutable copy `link()`/
/// `unlink()` update) rather than the YAML-shaped `graph_defn.graph`, so there's a
/// single source of truth for "who is connected to whom" while the sim is running.
pub fn update_connectors(
    g: Res<GraphDefinitionRes>,
    mut commands: Commands,
    query_changed: Query<(&NodeMarker, &Transform), Changed<Transform>>,
    query_all: Query<(&NodeMarker, &Transform)>,
    mut query_conn: Query<(Entity, &mut Path, &mut NodeConnector)>,
    // TEMPORARY - see systems::profiling's doc comment.
    mut prof: ResMut<crate::systems::profiling::ProfilingStats>,
) {
    let __prof_t0 = web_time::Instant::now(); // TEMPORARY
    (|| {
    if g.graph_defn.node_instances.is_empty() {
        for (entity, _, _) in query_conn.iter_mut() {
            commands.entity(entity).despawn_recursive();
        }
        return;
    }

    let mut all_node_loc = HashMap::<String, Vec3>::new();
    for (node, transform) in query_all.iter() {
        let mut pos: Vec3 = transform.translation;
        pos.z = 50.;
        all_node_loc.insert(node.node_name.clone(), pos);
    }

    // edges that should exist right now, keyed so A-B and B-A collapse to one entry.
    // Keys/values borrow from `node_instances`/`query_conn` rather than cloning -
    // this diff runs unconditionally every frame (it has to: `link()`/`unlink()`
    // mutate a node's `links` directly with no event firing, so there's no cheap
    // "topology changed" signal to gate it behind), and on a graph with a few
    // hundred edges the old `(String, String)`-keyed version was allocating on the
    // order of a few thousand `String`s per frame just to build and throw away two
    // `HashMap`s - a real, measurable cost distinct from (and larger than) the
    // path-retrace cost fixed above.
    let mut desired: HashMap<(&str, &str), (&str, &str)> = HashMap::new();
    for node in g.graph_defn.node_instances.iter() {
        for peer in node.links.iter() {
            if &node.name == peer {
                c_log!("Ignoring loopback: {}-{}", node.name, peer);
                continue;
            }
            if !all_node_loc.contains_key(&node.name) || !all_node_loc.contains_key(peer) {
                // one (or both) endpoints haven't been spawned as an entity yet -
                // pick this edge up on a later frame once they have been.
                continue;
            }
            let key = edge_key(&node.name, peer);
            desired
                .entry(key)
                .or_insert_with(|| (node.name.as_str(), peer.as_str()));
        }
    }

    let mut existing: HashMap<(&str, &str), Entity> = HashMap::new();
    for (entity, _, conn) in query_conn.iter() {
        existing.insert(edge_key(&conn.id1, &conn.id2), entity);
    }

    // drop connectors for edges that are no longer declared by either side
    for (key, entity) in existing.iter() {
        if !desired.contains_key(key) {
            commands.entity(*entity).despawn_recursive();
        }
    }

    // add connectors for newly-declared edges
    for (key, &(a, b)) in desired.iter() {
        if existing.contains_key(key) {
            continue;
        }
        let a_loc = all_node_loc.get(a).unwrap();
        let b_loc = all_node_loc.get(b).unwrap();
        let _ = generate_line(a_loc, b_loc, &g.graph_defn.graph_attrs, a, b, &mut commands);
    }

    // keep every surviving connector's path glued to its
    // endpoints as nodes move - but only the connectors actually touching a
    // node that moved *this frame*, not every connector in the graph. A
    // node's tick pulse (`node_pulse.rs`) animates `Transform.scale` for
    // 300ms on whichever node just ticked, so on a graph with many nodes on
    // short/staggered tick intervals `query_changed` is non-empty on
    // essentially every frame - retracing (bezier rebuild + lyon
    // re-tessellation) *every* connector on *every* frame regardless of
    // whether its own endpoints moved was a real, measurable bottleneck at a
    // few hundred nodes (a several-hundred-node stress-test graph).
    if !query_changed.is_empty() {
        let moved: HashSet<&str> = query_changed
            .iter()
            .map(|(m, _)| m.node_name.as_str())
            .collect();
        for (_, mut path, mut conn) in query_conn.iter_mut() {
            if !moved.contains(conn.id1.as_str()) && !moved.contains(conn.id2.as_str()) {
                continue;
            }
            let node1_loc = all_node_loc.get(&conn.id1);
            let node2_loc = all_node_loc.get(&conn.id2);
            if node1_loc.is_none() || node2_loc.is_none() {
                c_log!("Node not found for connector: {}-{}", conn.id1, conn.id2);
                continue;
            }
            *path = build_connector_path(
                node1_loc.unwrap().truncate(),
                node2_loc.unwrap().truncate(),
                g.graph_defn.graph_attrs.connector_style,
            );
            conn.path = path.0.clone();
            conn.walk_cache = walk_path(&conn.path);
        }
    }
    })(); // TEMPORARY
    prof.update_connectors_ms += __prof_t0.elapsed().as_secs_f64() * 1000.0; // TEMPORARY
}

fn generate_line(
    a: &Vec3,
    b: &Vec3,
    ga: &GraphAttrs,
    id1: &str,
    id2: &str,
    commands: &mut Commands,
) -> Entity {
    let path = build_connector_path(a.truncate(), b.truncate(), ga.connector_style);
    let walking_path = path.0.clone();
    let walk_cache = walk_path(&walking_path);
    let cc = ga.connection_color;

    commands
        .spawn((
            ShapeBundle {
                path,
                spatial: SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, 10.0)),
                ..default()
            },
            connector_stroke(cc),
            NodeConnector {
                id1: id1.to_string(),
                id2: id2.to_string(),
                path: walking_path,
                walk_cache,
                flash: 0.0,
            },
            Messages::default(),
        ))
        .id()
}

const BASE_WIDTH: f32 = 3.0;
const TRAFFIC_WIDTH_BONUS: f32 = 2.0;
const SELECTED_WIDTH_BONUS: f32 = 1.5;
const FLASH_WIDTH_BONUS: f32 = 2.0;
/// How long a delivery flash (`NodeConnector::flash`) takes to fully decay.
const FLASH_DECAY_SECS: f32 = 0.5;
/// Echoes the narration-bubble's indigo accent (see `explain_bubble`'s
/// `ACCENT_COLOR`) so a highlighted connector reads as the same app-wide
/// accent rather than an unrelated color.
const ACCENT_COLOR: Color = Color::srgb(0.49, 0.55, 0.98);

/// Recolors/re-weights each connector's stroke from its current state -
/// adjacency to the selected node, in-flight message traffic, and a
/// brief flash on delivery - by blending from the user-configured
/// `connection_color` (see `ui::graph_properties_viewer`'s color picker)
/// toward `ACCENT_COLOR`, rather than replacing it outright, so a custom
/// base color still comes through under the highlight.
pub fn update_connector_style(
    time: Res<Time>,
    g: Res<GraphDefinitionRes>,
    selected: Query<&SelectedNodeMarker>,
    mut connectors: Query<(&Messages, &mut NodeConnector, &mut Stroke)>,
    // TEMPORARY - see systems::profiling's doc comment.
    mut prof: ResMut<crate::systems::profiling::ProfilingStats>,
) {
    let __prof_t0 = web_time::Instant::now(); // TEMPORARY
    let selected_names: HashSet<&str> = selected.iter().map(|s| s.node_name.as_str()).collect();
    let base = g.graph_defn.graph_attrs.connection_color;

    for (messages, mut conn, mut stroke) in connectors.iter_mut() {
        conn.flash = (conn.flash - time.delta_seconds() / FLASH_DECAY_SECS).max(0.0);

        let adjacent_selected = selected_names.contains(conn.id1.as_str())
            || selected_names.contains(conn.id2.as_str());
        let traffic = (messages.msg_inflight.len() as f32 / 3.0).min(1.0);

        let mut width = BASE_WIDTH + traffic * TRAFFIC_WIDTH_BONUS;
        let mut accent = traffic * 0.25;
        if adjacent_selected {
            width += SELECTED_WIDTH_BONUS;
            accent = accent.max(0.4);
        }
        width += conn.flash * FLASH_WIDTH_BONUS;
        accent = accent.max(conn.flash * 0.9);
        let color = base.mix(&ACCENT_COLOR, accent.min(1.0));

        // `bevy_prototype_lyon` fully re-tessellates and allocates a brand-new
        // `Mesh` asset on *any* write to `Stroke` (it reacts to `Changed<Stroke>`,
        // and `Mut` derefs mark a component changed regardless of whether the
        // value actually differs) - on an idle connector (by far the common case
        // on a large graph) both `width` and `color` are identical to last
        // frame's, so writing unconditionally was re-tessellating every
        // connector's geometry every frame for nothing. Only touch `Stroke` when
        // something actually changed.
        if (stroke.options.line_width - width).abs() > 0.01 || stroke.color != color {
            stroke.options.line_width = width;
            stroke.color = color;
        }
    }
    prof.connector_style_ms += __prof_t0.elapsed().as_secs_f64() * 1000.0; // TEMPORARY
}
