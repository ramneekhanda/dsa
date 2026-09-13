use crate::c_log;
use bevy::{
    color::{Color, Srgba},
    prelude::*,
    time::Timer,
};
use rand::Rng;
use rhai::Dynamic;
use rhai::{CallFnOptions, Scope, AST};
use schemars::JsonSchema;
use serde::de::Error;
use serde::ser::Serializer;
use serde::{Deserialize, Deserializer, Serialize};
use std::time::Duration;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fmt::Debug,
};

fn white_color() -> Color {
    Srgba::hex("#FFFFFF").unwrap().into()
}

fn white_color_str() -> String {
    "#FFFFFF".to_string()
}

fn black_color() -> Color {
    Srgba::hex("#000000").unwrap().into()
}

fn black_color_str() -> String {
    "#000000".to_string()
}

fn deserialize_color<'de, D>(d: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(d).unwrap();

    let srgba = Srgba::hex(s.as_str());
    if srgba.is_ok() {
        let color = srgba.unwrap().into();
        Ok(color)
    } else {
        Err(Error::custom("Invalid color"))
    }
}

fn serialize_color<S>(c: &Color, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(format!("\"{}\"", c.to_srgba().to_hex()).as_str())
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, JsonSchema)]
pub struct GraphAttrs {
    #[schemars(with = "String", default = "white_color_str")]
    #[serde(
        default = "white_color",
        serialize_with = "serialize_color",
        deserialize_with = "deserialize_color"
    )]
    pub background: Color,

    #[schemars(with = "String", default = "black_color_str")]
    #[serde(
        default = "black_color",
        serialize_with = "serialize_color",
        deserialize_with = "deserialize_color"
    )]
    pub connection_color: Color,

    pub title: String,

    #[schemars(with = "String", default = "black_color_str")]
    #[serde(
        default = "black_color",
        serialize_with = "serialize_color",
        deserialize_with = "deserialize_color"
    )]
    pub text_color: Color,
}

impl Default for GraphAttrs {
    fn default() -> Self {
        GraphAttrs {
            background: white_color(),
            connection_color: black_color(),
            title: String::new(),
            text_color: black_color(),
        }
    }
}

/// Either a fixed tick interval in seconds, or a `{min, max}` range. A range with
/// `jitter: false` (the default) is resolved to one random value once, at node
/// creation, and stays constant for that instance's lifetime. `jitter: true` instead
/// re-picks a fresh random value within the range every time the timer fires.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(untagged)]
pub enum Ticks {
    Fixed(u64),
    Range {
        min: u64,
        max: u64,
        #[serde(default)]
        jitter: bool,
    },
}

impl Default for Ticks {
    fn default() -> Self {
        Ticks::Fixed(2)
    }
}

/// Resolves a `Ticks` value to a concrete interval in seconds, picking a random value
/// within the range for `Ticks::Range`.
pub fn resolve_ticks_secs(ticks: &Ticks) -> u64 {
    match ticks {
        Ticks::Fixed(n) => *n,
        Ticks::Range { min, max, .. } => rand::thread_rng().gen_range(*min..=*max),
    }
}

fn default_stroke_width() -> f32 {
    1.0
}

fn default_opacity() -> f32 {
    1.0
}

fn default_rounded_rect_radius() -> f32 {
    8.0
}

fn default_text_size() -> f32 {
    14.0
}

fn default_text_color() -> String {
    "#000000".to_string()
}

/// Matches `systems::node_system`'s `ICON_WIDTH`/`ICON_HEIGHT` - a
/// `TemplateShape::Icon` with no explicit `w`/`h` reads the same size as a
/// node's normal default icon sprite.
fn default_icon_size() -> f32 {
    64.0
}

/// A YAML-declarable equivalent of one shape in a Rhai handler's `draw([...])`
/// call (see `parser::draw`'s module doc for the shapes/fields this mirrors) -
/// lets a node type get a custom on-canvas look with no script at all, via
/// `Attrs::template`. Colors are the same `#rrggbb`/`#rgb`/`#rrggbbaa` hex
/// strings `draw()` accepts, parsed by the same `parser::draw::parse_color`.
///
/// String-valued fields (`text`, `fill`, `stroke`, `color`, `icon`) may embed
/// `{{param}}` placeholders, substituted per node instance before the shape is
/// drawn - see `NodeTemplateDef`'s doc comment for where `param`'s value comes
/// from. A template with no placeholders at all still works exactly as before
/// (the substitution pass is a no-op on a string containing no `{{`).
///
/// Otherwise static - a template can't reference a node's `globals` or branch
/// on logic the way a live `draw()` call can (there's no script evaluation
/// involved, only string substitution). A node type that also has a Rhai `fn`
/// and calls `draw()` from it will have that call *replace* the
/// template-seeded overlay the first time it runs (same "draw() replaces the
/// overlay" rule documented on `Node::overlay`), so a template is really a
/// default/initial look, not a persistent base layer underneath dynamic
/// shapes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "shape", rename_all = "snake_case")]
pub enum TemplateShape {
    Rect {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        #[serde(default)]
        radius: f32,
        #[serde(default)]
        fill: Option<String>,
        #[serde(default)]
        stroke: Option<String>,
        #[serde(default = "default_stroke_width")]
        stroke_width: f32,
        #[serde(default = "default_opacity")]
        opacity: f32,
    },
    /// Identical to `Rect` - `radius` just defaults to `8.0` instead of
    /// `0.0`, so `shape: roundedrect` alone (no `radius` given) still reads
    /// as rounded. `radius: 0` on a `roundedrect` is a valid (if pointless)
    /// way to square it back off.
    #[serde(rename = "roundedrect")]
    RoundedRect {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        #[serde(default = "default_rounded_rect_radius")]
        radius: f32,
        #[serde(default)]
        fill: Option<String>,
        #[serde(default)]
        stroke: Option<String>,
        #[serde(default = "default_stroke_width")]
        stroke_width: f32,
        #[serde(default = "default_opacity")]
        opacity: f32,
    },
    Circle {
        x: f32,
        y: f32,
        r: f32,
        #[serde(default)]
        fill: Option<String>,
        #[serde(default)]
        stroke: Option<String>,
        #[serde(default = "default_stroke_width")]
        stroke_width: f32,
        #[serde(default = "default_opacity")]
        opacity: f32,
    },
    Line {
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        stroke: String,
        #[serde(default = "default_stroke_width")]
        stroke_width: f32,
    },
    Polygon {
        points: Vec<[f32; 2]>,
        /// `true` connects the last point back to the first (a closed
        /// polygon); `false` leaves it open (a polyline) - the two shape
        /// names `draw()` accepts as sugar for this are collapsed to one
        /// explicit flag here.
        closed: bool,
        #[serde(default)]
        fill: Option<String>,
        #[serde(default)]
        stroke: Option<String>,
        #[serde(default = "default_stroke_width")]
        stroke_width: f32,
    },
    Text {
        x: f32,
        y: f32,
        text: String,
        #[serde(default = "default_text_size")]
        size: f32,
        #[serde(default = "default_text_color")]
        color: String,
    },
    /// An icon-registry image at an arbitrary node-local position/size -
    /// typically `icon: "{{icon}}"`, so the template draws whichever icon
    /// each node type/instance actually binds (see `NodeTemplateDef`), rather
    /// than one hardcoded icon shared by every node type that uses it.
    ///
    /// A template is the *whole* node visual once it's set at all (see
    /// `Attrs::template`'s doc comment) - the default icon sprite
    /// `node_system::spawn_node` would otherwise draw is skipped entirely,
    /// so a template with no `Icon` shape of its own means that node type
    /// simply has no icon. And since a script's own `draw()` call replaces
    /// the *entire* overlay a template seeded, not just the parts that
    /// call is meant to update, a handler that calls `draw()` to refresh
    /// some other live bit (a status badge, say) needs to re-include its own `Icon`
    /// shape in that same call, or the icon disappears the moment the
    /// handler first runs.
    Icon {
        x: f32,
        y: f32,
        #[serde(default = "default_icon_size")]
        w: f32,
        #[serde(default = "default_icon_size")]
        h: f32,
        icon: String,
    },
}

impl TemplateShape {
    /// Converts to the same `DrawCmd` a parsed `draw()` call produces, so a
    /// YAML template renders through the exact same
    /// `systems::node_overlay` pipeline. Returns `None` only for an
    /// unparsable color string (the YAML schema already rules out anything
    /// else malformed, unlike `draw()`'s free-form Rhai maps).
    pub fn to_draw_cmd(&self) -> Option<crate::parser::draw::DrawCmd> {
        use crate::parser::draw::{parse_color, DrawCmd};
        Some(match self {
            TemplateShape::Rect {
                x,
                y,
                w,
                h,
                radius,
                fill,
                stroke,
                stroke_width,
                opacity,
            }
            | TemplateShape::RoundedRect {
                x,
                y,
                w,
                h,
                radius,
                fill,
                stroke,
                stroke_width,
                opacity,
            } => DrawCmd::Rect {
                x: *x,
                y: *y,
                w: *w,
                h: *h,
                radius: *radius,
                paint: paint_of(fill, stroke, *stroke_width, *opacity)?,
            },
            TemplateShape::Circle {
                x,
                y,
                r,
                fill,
                stroke,
                stroke_width,
                opacity,
            } => DrawCmd::Circle {
                x: *x,
                y: *y,
                r: *r,
                paint: paint_of(fill, stroke, *stroke_width, *opacity)?,
            },
            TemplateShape::Line {
                x1,
                y1,
                x2,
                y2,
                stroke,
                stroke_width,
            } => DrawCmd::Line {
                x1: *x1,
                y1: *y1,
                x2: *x2,
                y2: *y2,
                color: parse_color(stroke)?,
                width: *stroke_width,
            },
            TemplateShape::Polygon {
                points,
                closed,
                fill,
                stroke,
                stroke_width,
            } => DrawCmd::Polygon {
                points: points.iter().map(|[x, y]| Vec2::new(*x, *y)).collect(),
                closed: *closed,
                paint: paint_of(fill, stroke, *stroke_width, 1.0)?,
            },
            TemplateShape::Text {
                x,
                y,
                text,
                size,
                color,
            } => DrawCmd::Text {
                x: *x,
                y: *y,
                text: text.clone(),
                size: *size,
                color: parse_color(color)?,
            },
            TemplateShape::Icon { x, y, w, h, icon } => DrawCmd::Icon {
                x: *x,
                y: *y,
                w: *w,
                h: *h,
                icon: icon.clone(),
            },
        })
    }

    /// Returns a copy of `self` with every `{{param}}` placeholder in its
    /// string-valued fields replaced by `params[param]` - unmatched
    /// placeholders (a typo'd param name, say) are left verbatim rather than
    /// silently dropped, so the mistake is visible on canvas instead of
    /// invisible. A field with no `{{` at all is untouched.
    pub fn substituted(&self, params: &HashMap<String, String>) -> TemplateShape {
        fn sub(s: &str, params: &HashMap<String, String>) -> String {
            if !s.contains("{{") {
                return s.to_string();
            }
            let mut out = s.to_string();
            for (k, v) in params {
                out = out.replace(&format!("{{{{{}}}}}", k), v);
            }
            out
        }
        match self {
            TemplateShape::Rect {
                x,
                y,
                w,
                h,
                radius,
                fill,
                stroke,
                stroke_width,
                opacity,
            } => TemplateShape::Rect {
                x: *x,
                y: *y,
                w: *w,
                h: *h,
                radius: *radius,
                fill: fill.as_deref().map(|s| sub(s, params)),
                stroke: stroke.as_deref().map(|s| sub(s, params)),
                stroke_width: *stroke_width,
                opacity: *opacity,
            },
            TemplateShape::RoundedRect {
                x,
                y,
                w,
                h,
                radius,
                fill,
                stroke,
                stroke_width,
                opacity,
            } => TemplateShape::RoundedRect {
                x: *x,
                y: *y,
                w: *w,
                h: *h,
                radius: *radius,
                fill: fill.as_deref().map(|s| sub(s, params)),
                stroke: stroke.as_deref().map(|s| sub(s, params)),
                stroke_width: *stroke_width,
                opacity: *opacity,
            },
            TemplateShape::Circle {
                x,
                y,
                r,
                fill,
                stroke,
                stroke_width,
                opacity,
            } => TemplateShape::Circle {
                x: *x,
                y: *y,
                r: *r,
                fill: fill.as_deref().map(|s| sub(s, params)),
                stroke: stroke.as_deref().map(|s| sub(s, params)),
                stroke_width: *stroke_width,
                opacity: *opacity,
            },
            TemplateShape::Line {
                x1,
                y1,
                x2,
                y2,
                stroke,
                stroke_width,
            } => TemplateShape::Line {
                x1: *x1,
                y1: *y1,
                x2: *x2,
                y2: *y2,
                stroke: sub(stroke, params),
                stroke_width: *stroke_width,
            },
            TemplateShape::Polygon {
                points,
                closed,
                fill,
                stroke,
                stroke_width,
            } => TemplateShape::Polygon {
                points: points.clone(),
                closed: *closed,
                fill: fill.as_deref().map(|s| sub(s, params)),
                stroke: stroke.as_deref().map(|s| sub(s, params)),
                stroke_width: *stroke_width,
            },
            TemplateShape::Text {
                x,
                y,
                text,
                size,
                color,
            } => TemplateShape::Text {
                x: *x,
                y: *y,
                text: sub(text, params),
                size: *size,
                color: sub(color, params),
            },
            TemplateShape::Icon { x, y, w, h, icon } => TemplateShape::Icon {
                x: *x,
                y: *y,
                w: *w,
                h: *h,
                icon: sub(icon, params),
            },
        }
    }
}

/// Shared by every `TemplateShape` variant with fill/stroke - parses both
/// (either may be absent) and applies `opacity` the same way
/// `parser::draw::paint_of` does for a Rhai `draw()` call.
fn paint_of(
    fill: &Option<String>,
    stroke: &Option<String>,
    stroke_width: f32,
    opacity: f32,
) -> Option<crate::parser::draw::Paint> {
    use crate::parser::draw::{parse_color, Paint};
    let opacity = opacity.clamp(0.0, 1.0);
    let with_opacity = |c: Color| {
        let mut s = c.to_srgba();
        s.alpha *= opacity;
        Color::from(s)
    };
    let fill = match fill {
        Some(s) => Some(with_opacity(parse_color(s)?)),
        None => None,
    };
    let stroke = match stroke {
        Some(s) => Some(with_opacity(parse_color(s)?)),
        None => None,
    };
    Some(Paint {
        fill,
        stroke,
        stroke_width: stroke_width.max(0.0),
    })
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Attrs {
    #[serde(default)]
    pub ticks: Ticks,
    /// Id of an entry in the graph's top-level `icons` list, used as this node type's
    /// on-canvas icon.
    pub icon: Option<String>,
    /// A static custom look for this node type, declared directly in YAML
    /// instead of via a Rhai `draw()` call - see `TemplateShape`'s doc
    /// comment. Renders on top of the icon/label, in the same node-local
    /// coordinate space `draw()` uses (origin at node center, y up). Can be
    /// combined with `template_ref` (that template's shapes are applied
    /// first, these on top of them) or used alone for a one-off look not
    /// worth naming and sharing.
    #[serde(default)]
    pub template: Option<Vec<TemplateShape>>,
    /// Id of an entry in the graph's top-level `node_templates` list (see
    /// `NodeTemplateDef`) - reuses that named template's shapes for this
    /// node type instead of (or as a base for, if `template` is also set)
    /// writing them out again inline. Resolved once at parse time
    /// (`parse_graph2`), the same way `icon` is validated against `icons`.
    #[serde(default)]
    pub template_ref: Option<String>,
    /// Explicit `{{param}}` -> value bindings for this node type's template
    /// (own `template` and/or a referenced `template_ref`) - merged over the
    /// two builtin auto-bound params every node gets for free: `node_name`
    /// (this specific node *instance*'s own name, so a template shared by
    /// several instances still labels each one correctly) and `icon` (this
    /// node type's own `attrs.icon`, if set). An entry here overrides the
    /// matching builtin if both set the same key - e.g. to have a template
    /// show a different icon than the node's actual on-canvas icon. See
    /// `NodeTemplateDef`'s doc comment for the overall mechanism.
    #[serde(default)]
    pub template_params: Option<HashMap<String, String>>,
}

impl Default for Attrs {
    fn default() -> Self {
        Attrs {
            ticks: Ticks::default(),
            icon: None,
            template: None,
            template_ref: None,
            template_params: None,
        }
    }
}

/// One reusable named entry in the graph's top-level `node_templates` list -
/// referenced by `Attrs::template_ref`, the same pattern `IconDef`/`icons`
/// already uses for sharing an icon across node types instead of repeating a
/// URL. Exists purely to be referenced; has no meaning on its own.
///
/// A single template can give a consistent look to every node type that uses
/// it - e.g. an icon plus a name label laid out the same way for every node
/// in the graph - by writing its shapes generically, in terms of `{{param}}`
/// placeholders (see `TemplateShape`'s doc comment), instead of one specific
/// icon/label. `params` documents which placeholders the template expects,
/// for the YAML author's own reference and for the JSON-schema-driven editor;
/// it isn't itself checked against what a node type actually binds - an
/// unresolved `{{param}}` just renders literally rather than erroring, so a
/// typo is a visible canvas bug, not a load-time failure.
///
/// Two params are always available with no binding needed at all -
/// `node_name` (the specific node instance's own name) and `icon` (that node
/// type's own `attrs.icon`) - see `Attrs::template_params` for how those
/// auto-bound values combine with any explicit bindings.
#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone, JsonSchema)]
pub struct NodeTemplateDef {
    pub id: String,
    /// Documents the `{{param}}` names this template's shapes reference,
    /// beyond the always-available `node_name`/`icon` builtins - purely
    /// informational, not validated.
    #[serde(default)]
    pub params: Vec<String>,
    pub shapes: Vec<TemplateShape>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum ParamType {
    Bool {
        default: bool,
    },
    Float {
        min: f64,
        max: f64,
        default: f64,
    },
    Integer {
        min: i64,
        max: i64,
        default: i64,
    },
    String {
        default: String,
    },
    Option {
        values: HashSet<String>,
        default: String,
    },
}

impl Default for ParamType {
    fn default() -> Self {
        ParamType::String {
            default: "".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct NodeParams {
    pub name: String,
    #[serde(flatten)]
    pub param_type: ParamType,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct NodeType {
    pub id: String,
    #[serde(rename = "fn")]
    pub func: Option<String>,
    #[serde(default)]
    pub attrs: Attrs,
    pub params: Option<Vec<NodeParams>>,
    #[serde(skip)]
    pub ast: AST,
}

impl std::cmp::PartialEq for NodeType {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.func == other.func
            && self.attrs == other.attrs
            && self.params == other.params
    }
}

#[derive(Debug, Default, Clone)]
pub struct Node {
    pub name: String,
    pub node_data: NodeType,
    pub links: Vec<String>,
    pub timer: Timer,

    pub ast: AST, //TODO: change this to reference
    pub scope: Scope<'static>,
    pub state: Dynamic,

    /// Custom canvas overlay for this node, set from a Rhai handler via `draw([...])`.
    /// Persists until the node calls `draw()` again; `overlay_dirty` flags a pending
    /// re-render for `systems::node_overlay`.
    pub overlay: Vec<crate::parser::draw::DrawCmd>,
    pub overlay_dirty: bool,
}

impl std::cmp::PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.node_data == other.node_data
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize, Clone, JsonSchema)]
pub struct NodeConnection {
    pub name: String,
    pub node_type: String,
    pub links: Vec<String>,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone, JsonSchema)]
pub struct IconDef {
    pub id: String,
    pub url: String,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone, JsonSchema)]
pub struct GraphDefinition {
    pub node_types: Vec<NodeType>,
    pub graph: Vec<NodeConnection>,
    pub graph_attrs: GraphAttrs,
    #[serde(default)]
    pub icons: Vec<IconDef>,
    /// Named, reusable shape lists a node type's `attrs.template_ref` can
    /// point at instead of repeating the same `attrs.template` shapes
    /// inline across several node types - see `NodeTemplateDef`.
    #[serde(default)]
    pub node_templates: Vec<NodeTemplateDef>,

    #[serde(skip)]
    pub node_instances: Vec<Node>,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize, Clone, JsonSchema)]
pub struct File {
    #[serde(default)]
    pub graph_defn: GraphDefinition,

    #[serde(skip)]
    pub timers: HashMap<String, Timer>,
}

pub fn parse_graph2(graph_code: &String) -> Result<File, serde_yaml::Error> {
    let mut engine = rhai::Engine::new();
    // Rhai's default nesting limit inside a function is only 32 levels, which a
    // realistic handler (a map literal with an inline `if`, say) can trip. Raise
    // it well clear of hand-written scripts while still bounding pathological input.
    engine.set_max_expr_depths(256, 256);

    let data: Result<File, serde_yaml::Error> = serde_yaml::from_str(&graph_code);
    if let Ok(mut m_data) = data {
        for node_type in m_data.graph_defn.node_types.iter_mut() {
            let res = compile_ast(&engine, node_type);
            if res.is_err() {
                let e = res.err().unwrap();
                return Err(serde_yaml::Error::custom(
                    format!(
                        "Error compiling function for node type {} with error: {:?}",
                        node_type.id, e
                    )
                    .as_str(),
                ));
            }
            if let Some(icon_id) = &node_type.attrs.icon {
                if !m_data.graph_defn.icons.iter().any(|i| &i.id == icon_id) {
                    return Err(serde_yaml::Error::custom(
                        format!(
                            "Icon '{}' referenced by node type '{}' not found in icons list",
                            icon_id, node_type.id
                        )
                        .as_str(),
                    ));
                }
            }
            if let Some(template_ref) = &node_type.attrs.template_ref {
                let Some(named) = m_data
                    .graph_defn
                    .node_templates
                    .iter()
                    .find(|t| &t.id == template_ref)
                else {
                    return Err(serde_yaml::Error::custom(
                        format!(
                            "Template '{}' referenced by node type '{}' not found in node_templates list",
                            template_ref, node_type.id
                        )
                        .as_str(),
                    ));
                };
                // The named template's shapes first, then any inline
                // `attrs.template` shapes on top of them - lets a node type
                // start from a shared base and add its own per-type touches
                // without repeating the base itself.
                let mut shapes = named.shapes.clone();
                if let Some(inline) = node_type.attrs.template.take() {
                    shapes.extend(inline);
                }
                node_type.attrs.template = Some(shapes);
            }
            if let Ticks::Range { min, max, .. } = &node_type.attrs.ticks {
                if min > max {
                    return Err(serde_yaml::Error::custom(
                        format!(
                            "Invalid ticks range for node type '{}': min ({}) > max ({})",
                            node_type.id, min, max
                        )
                        .as_str(),
                    ));
                }
            }
        }
        for node in m_data.graph_defn.graph.iter() {
            let type_data = m_data
                .graph_defn
                .node_types
                .iter()
                .find(|x| x.id == node.node_type);
            if let Some(data_w_type) = type_data {
                let n =
                    instantiate_node(&engine, data_w_type, node.name.clone(), node.links.clone());
                m_data.graph_defn.node_instances.push(n);
            } else {
                return Err(serde_yaml::Error::custom(
                    format!("Node type not found for type {}", node.node_type).as_str(),
                ));
            }
        }
        return Ok(m_data);
    }
    data
}

/// Builds a fresh `Node` instance of `node_type` and runs its `on_init` once. Used
/// both at initial YAML parse time (a bare, unconfigured `engine` - `log`/`send`/
/// `draw`/`spawn`/etc all silently no-op there, matching prior behavior) and by
/// `rhai_engine::apply_spawns` for a script's runtime `spawn()` (the fully-registered
/// engine there, so `draw()` in `on_init` paints the new node's overlay immediately).
pub fn instantiate_node(
    engine: &rhai::Engine,
    node_type: &NodeType,
    name: String,
    links: Vec<String>,
) -> Node {
    let mut n = Node {
        name,
        node_data: node_type.clone(),
        timer: Timer::new(
            Duration::from_secs(resolve_ticks_secs(&node_type.attrs.ticks)),
            TimerMode::Repeating,
        ),
        links,
        ast: node_type.ast.clone(),
        scope: Scope::new(),
        state: Dynamic::from_map(BTreeMap::new()),
        overlay: Vec::new(),
        overlay_dirty: false,
    };
    // Seed the overlay from a YAML-declared `attrs.template` (see
    // `TemplateShape`'s doc comment) *before* `on_init` runs - a script's
    // own `draw()` call in `on_init` replaces this outright (same as any
    // later `draw()` call), so a template only actually shows for a node
    // type that never calls `draw()` at all, or hasn't yet by the time
    // this first render happens.
    if let Some(template) = &node_type.attrs.template {
        let params = template_params(&n.name, &node_type.attrs);
        n.overlay = template
            .iter()
            .map(|s| s.substituted(&params))
            .filter_map(|s| s.to_draw_cmd())
            .take(crate::parser::draw::MAX_SHAPES_PER_NODE)
            .collect();
        n.overlay_dirty = true;
    }
    init_scope(engine, &mut n);
    n
}

/// Builds the `{{param}}` -> value map used to render `instance_name`'s
/// template (see `TemplateShape::substituted` / `NodeTemplateDef`'s doc
/// comment): the two auto-bound builtins (`node_name`, `icon`) first, then
/// any explicit `attrs.template_params` layered on top so they can override
/// either builtin (or add params of their own the template refers to).
fn template_params(instance_name: &str, attrs: &Attrs) -> HashMap<String, String> {
    let mut params = HashMap::new();
    params.insert("node_name".to_string(), instance_name.to_string());
    if let Some(icon) = &attrs.icon {
        params.insert("icon".to_string(), icon.clone());
    }
    if let Some(explicit) = &attrs.template_params {
        for (k, v) in explicit {
            params.insert(k.clone(), v.clone());
        }
    }
    params
}

pub fn init_scope(engine: &rhai::Engine, node: &mut Node) {
    let scope = &mut node.scope;
    scope.push_constant("node_name", node.name.clone());
    let links: Dynamic = node.links.clone().into();
    scope.push_constant("links", links);
    let init_size = scope.len();
    let options = CallFnOptions::new().eval_ast(false).rewind_scope(false);

    scope.set_value("globals", node.state.clone());
    let _ = engine.call_fn_with_options::<()>(options, scope, &node.ast, "on_init", ());
    node.state = scope.get_value::<Dynamic>("globals").unwrap();

    c_log!("scope size: {}", scope.len());
    c_log!("scope: {:?}", scope);
    c_log!("globals {:?}", node.state);
    scope.rewind(init_size);
}

pub fn compile_ast(engine: &rhai::Engine, node: &mut NodeType) -> Result<bool, rhai::ParseError> {
    c_log!("Compiling code for node type {}", node.id);
    if node.func.is_none() {
        c_log!("No code for node type {}", node.id);
        return Ok(true);
    }
    let ast = engine.compile(node.func.as_ref().unwrap());
    if ast.is_ok() {
        c_log!("code compiled for node type {}", node.id);
        node.ast = ast.unwrap();
        return Ok(true);
    }
    Err(ast.err().unwrap())
}
