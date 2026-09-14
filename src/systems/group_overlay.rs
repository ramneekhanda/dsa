use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::parser::graphv2::parse_color_str;
use crate::resources::common_assets::{CommonAssets, ResourceType};
use crate::resources::graph_def::{GraphChange, GraphDefinitionRes};
use crate::systems::layout::compute_graph_layout;

#[derive(Component)]
pub struct GroupBoxMarker {
    pub group_id: String,
}

/// Z-index of group container boxes - well behind nodes (z=100+) and connectors (z=10+),
/// but in front of the background canvas grid (z=0).
const GROUP_BOX_Z: f32 = 5.0;

/// Updates and spawns visual group container boxes when the graph definition changes.
pub fn update_group_boxes(
    mut commands: Commands,
    graph_defn: Res<GraphDefinitionRes>,
    mut change_reader: EventReader<GraphChange>,
    existing_boxes: Query<Entity, With<GroupBoxMarker>>,
    ca: Res<CommonAssets>,
) {
    if change_reader.read().count() == 0 {
        return;
    }

    // Despawn existing group boxes
    for entity in existing_boxes.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let gd = &graph_defn.graph_defn;
    if gd.groups.is_empty() {
        return;
    }

    let layout = compute_graph_layout(gd);
    let mut font: Handle<Font> = Default::default();
    if let Some(ResourceType::FontHandle(f)) = ca.resource_map.get("default_font") {
        font = f.clone();
    }

    for gb in layout.group_boxes {
        // Only render if box is not explicitly disabled
        if gb.style.r#box == Some(false) {
            continue;
        }

        let radius = gb.style.radius.unwrap_or(14.0);
        let border_width = gb.style.border_width.unwrap_or(1.5);

        let border_color = gb
            .style
            .border
            .as_deref()
            .and_then(parse_color_str)
            .unwrap_or(Color::srgba(0.58, 0.64, 0.72, 0.45)); // Slate 400

        let bg_color = gb
            .style
            .bg
            .as_deref()
            .and_then(parse_color_str)
            .unwrap_or(Color::srgba(0.95, 0.96, 0.98, 0.4)); // Translucent tint

        let rect_shape = shapes::RoundedPolygon {
            points: vec![
                Vec2::new(-gb.size.x / 2.0, -gb.size.y / 2.0),
                Vec2::new(gb.size.x / 2.0, -gb.size.y / 2.0),
                Vec2::new(gb.size.x / 2.0, gb.size.y / 2.0),
                Vec2::new(-gb.size.x / 2.0, gb.size.y / 2.0),
            ],
            radius,
            closed: true,
        };

        let box_mesh = GeometryBuilder::build_as(&rect_shape);

        let parent = commands
            .spawn((
                ShapeBundle {
                    path: box_mesh,
                    spatial: SpatialBundle::from_transform(Transform::from_xyz(
                        gb.center.x,
                        gb.center.y,
                        GROUP_BOX_Z,
                    )),
                    ..default()
                },
                Fill::color(bg_color),
                Stroke::new(border_color, border_width),
                GroupBoxMarker {
                    group_id: gb.group_id.clone(),
                },
            ))
            .id();

        // Render group title label if present
        if let Some(ref title) = gb.title {
            let title_color = border_color.with_alpha(0.9);
            let label = commands
                .spawn(Text2dBundle {
                    text: Text::from_section(
                        title.to_uppercase(),
                        TextStyle {
                            font: font.clone(),
                            font_size: 11.0,
                            color: title_color,
                        },
                    ),
                    text_anchor: bevy::sprite::Anchor::TopLeft,
                    transform: Transform::from_xyz(
                        -gb.size.x / 2.0 + 14.0,
                        gb.size.y / 2.0 - 10.0,
                        0.01,
                    ),
                    ..default()
                })
                .id();
            commands.entity(parent).add_child(label);
        }
    }
}
