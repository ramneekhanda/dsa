use crate::parser::graphv2::{File, GraphDefinition};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// An import definition allowing inclusion of external or preset YAML modules.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, JsonSchema)]
pub struct ImportDef {
    /// Remote URL (e.g. `https://...`) or preset identifier (e.g. `theme:cyberpunk`, `stdlib:load_balancer`)
    pub from: String,
    /// Optional list of specific node type IDs to import from this module
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub import: Option<Vec<String>>,
    /// Optional alias / renaming for imported node types
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#as: Option<String>,
}

pub const PRESET_THEME_CYBERPUNK: &str = r##"
graph_defn:
  graph_attrs:
    background: "#070b19"
    connection_color: "#00f0ff"
    text_color: "#e0f7fa"
    message_theme:
      shape: chamfered
      bg: "#060d17"
      stroke: "#00f5ff"
      stroke_width: 1.5
      text_color: "#00f5ff"
      font_size: 14.0
  node_templates:
    - id: cyber_hud
      shapes:
        - shape: polygon
          points:
            - [-48.0, 32.0]
            - [48.0, 32.0]
            - [60.0, 20.0]
            - [60.0, -20.0]
            - [48.0, -32.0]
            - [-48.0, -32.0]
            - [-60.0, -20.0]
            - [-60.0, 20.0]
          closed: true
          fill: "#090d16"
          stroke: "{{neon_border}}"
          stroke_width: 1.5
        - shape: line
          x1: -32.0
          y1: 26.0
          x2: 32.0
          y2: 26.0
          stroke: "{{neon_accent}}"
          stroke_width: 2.0
        - shape: circle
          x: -38.0
          y: 0.0
          r: 13.0
          fill: "#0f172a"
          stroke: "{{neon_accent}}"
          stroke_width: 1.2
        - shape: icon
          x: -38.0
          y: 0.0
          w: 18.0
          h: 18.0
          icon: "{{icon}}"
        - shape: text
          x: 12.0
          y: 6.0
          text: "{{node_name}}"
          size: 9.5
          color: "#f0fdf4"
        - shape: text
          x: 12.0
          y: -6.0
          text: "{{telemetry}}"
          size: 7.5
          color: "{{neon_accent}}"
        - shape: roundedrect
          x: 0.0
          y: -18.0
          w: 86.0
          h: 10.0
          radius: 2.0
          fill: "{{badge_bg}}"
        - shape: text
          x: 0.0
          y: -18.0
          text: "{{status_code}}"
          size: 6.5
          color: "{{badge_color}}"
        - shape: progress
          style: segmented
          x: 0.0
          y: -26.0
          w: 64.0
          h: 3.0
          segments: 6
          gap: 2.0
          track_color: "#0f172a"
          fill_color: "{{neon_accent}}"
    - id: default
      shapes:
        - shape: polygon
          points:
            - [-48.0, 32.0]
            - [48.0, 32.0]
            - [60.0, 20.0]
            - [60.0, -20.0]
            - [48.0, -32.0]
            - [-48.0, -32.0]
            - [-60.0, -20.0]
            - [-60.0, 20.0]
          closed: true
          fill: "#090d16"
          stroke: "{{neon_border}}"
          stroke_width: 1.5
        - shape: line
          x1: -32.0
          y1: 26.0
          x2: 32.0
          y2: 26.0
          stroke: "{{neon_accent}}"
          stroke_width: 2.0
        - shape: circle
          x: -38.0
          y: 0.0
          r: 13.0
          fill: "#0f172a"
          stroke: "{{neon_accent}}"
          stroke_width: 1.2
        - shape: icon
          x: -38.0
          y: 0.0
          w: 18.0
          h: 18.0
          icon: "{{icon}}"
        - shape: text
          x: 12.0
          y: 6.0
          text: "{{node_name}}"
          size: 9.5
          color: "#f0fdf4"
        - shape: text
          x: 12.0
          y: -6.0
          text: "{{telemetry}}"
          size: 7.5
          color: "{{neon_accent}}"
        - shape: roundedrect
          x: 0.0
          y: -18.0
          w: 86.0
          h: 10.0
          radius: 2.0
          fill: "{{badge_bg}}"
        - shape: text
          x: 0.0
          y: -18.0
          text: "{{status_code}}"
          size: 6.5
          color: "{{badge_color}}"
        - shape: progress
          style: segmented
          x: 0.0
          y: -26.0
          w: 64.0
          h: 3.0
          segments: 6
          gap: 2.0
          track_color: "#0f172a"
          fill_color: "{{neon_accent}}"
    - id: card
      shapes:
        - shape: polygon
          points:
            - [-48.0, 32.0]
            - [48.0, 32.0]
            - [60.0, 20.0]
            - [60.0, -20.0]
            - [48.0, -32.0]
            - [-48.0, -32.0]
            - [-60.0, -20.0]
            - [-60.0, 20.0]
          closed: true
          fill: "#090d16"
          stroke: "{{neon_border}}"
          stroke_width: 1.5
        - shape: line
          x1: -32.0
          y1: 26.0
          x2: 32.0
          y2: 26.0
          stroke: "{{neon_accent}}"
          stroke_width: 2.0
        - shape: circle
          x: -38.0
          y: 0.0
          r: 13.0
          fill: "#0f172a"
          stroke: "{{neon_accent}}"
          stroke_width: 1.2
        - shape: icon
          x: -38.0
          y: 0.0
          w: 18.0
          h: 18.0
          icon: "{{icon}}"
        - shape: text
          x: 12.0
          y: 6.0
          text: "{{node_name}}"
          size: 9.5
          color: "#f0fdf4"
        - shape: text
          x: 12.0
          y: -6.0
          text: "{{telemetry}}"
          size: 7.5
          color: "{{neon_accent}}"
        - shape: roundedrect
          x: 0.0
          y: -18.0
          w: 86.0
          h: 10.0
          radius: 2.0
          fill: "{{badge_bg}}"
        - shape: text
          x: 0.0
          y: -18.0
          text: "{{status_code}}"
          size: 6.5
          color: "{{badge_color}}"
        - shape: progress
          style: segmented
          x: 0.0
          y: -26.0
          w: 64.0
          h: 3.0
          segments: 6
          gap: 2.0
          track_color: "#0f172a"
          fill_color: "{{neon_accent}}"
    - id: cloud_card
      shapes:
        - shape: polygon
          points:
            - [-48.0, 32.0]
            - [48.0, 32.0]
            - [60.0, 20.0]
            - [60.0, -20.0]
            - [48.0, -32.0]
            - [-48.0, -32.0]
            - [-60.0, -20.0]
            - [-60.0, 20.0]
          closed: true
          fill: "#090d16"
          stroke: "{{neon_border}}"
          stroke_width: 1.5
        - shape: line
          x1: -32.0
          y1: 26.0
          x2: 32.0
          y2: 26.0
          stroke: "{{neon_accent}}"
          stroke_width: 2.0
        - shape: circle
          x: -38.0
          y: 0.0
          r: 13.0
          fill: "#0f172a"
          stroke: "{{neon_accent}}"
          stroke_width: 1.2
        - shape: icon
          x: -38.0
          y: 0.0
          w: 18.0
          h: 18.0
          icon: "{{icon}}"
        - shape: text
          x: 12.0
          y: 6.0
          text: "{{node_name}}"
          size: 9.5
          color: "#f0fdf4"
        - shape: text
          x: 12.0
          y: -6.0
          text: "{{telemetry}}"
          size: 7.5
          color: "{{neon_accent}}"
        - shape: roundedrect
          x: 0.0
          y: -18.0
          w: 86.0
          h: 10.0
          radius: 2.0
          fill: "{{badge_bg}}"
        - shape: text
          x: 0.0
          y: -18.0
          text: "{{status_code}}"
          size: 6.5
          color: "{{badge_color}}"
        - shape: progress
          style: segmented
          x: 0.0
          y: -26.0
          w: 64.0
          h: 3.0
          segments: 6
          gap: 2.0
          track_color: "#0f172a"
          fill_color: "{{neon_accent}}"
    - id: rack_blade
      shapes:
        - shape: polygon
          points:
            - [-48.0, 32.0]
            - [48.0, 32.0]
            - [60.0, 20.0]
            - [60.0, -20.0]
            - [48.0, -32.0]
            - [-48.0, -32.0]
            - [-60.0, -20.0]
            - [-60.0, 20.0]
          closed: true
          fill: "#090d16"
          stroke: "{{neon_border}}"
          stroke_width: 1.5
        - shape: line
          x1: -32.0
          y1: 26.0
          x2: 32.0
          y2: 26.0
          stroke: "{{neon_accent}}"
          stroke_width: 2.0
        - shape: circle
          x: -38.0
          y: 0.0
          r: 13.0
          fill: "#0f172a"
          stroke: "{{neon_accent}}"
          stroke_width: 1.2
        - shape: icon
          x: -38.0
          y: 0.0
          w: 18.0
          h: 18.0
          icon: "{{icon}}"
        - shape: text
          x: 12.0
          y: 6.0
          text: "{{node_name}}"
          size: 9.5
          color: "#f0fdf4"
        - shape: text
          x: 12.0
          y: -6.0
          text: "{{telemetry}}"
          size: 7.5
          color: "{{neon_accent}}"
        - shape: roundedrect
          x: 0.0
          y: -18.0
          w: 86.0
          h: 10.0
          radius: 2.0
          fill: "{{badge_bg}}"
        - shape: text
          x: 0.0
          y: -18.0
          text: "{{status_code}}"
          size: 6.5
          color: "{{badge_color}}"
        - shape: progress
          style: segmented
          x: 0.0
          y: -26.0
          w: 64.0
          h: 3.0
          segments: 6
          gap: 2.0
          track_color: "#0f172a"
          fill_color: "{{neon_accent}}"
    - id: minimal_pill
      shapes:
        - shape: polygon
          points:
            - [-48.0, 32.0]
            - [48.0, 32.0]
            - [60.0, 20.0]
            - [60.0, -20.0]
            - [48.0, -32.0]
            - [-48.0, -32.0]
            - [-60.0, -20.0]
            - [-60.0, 20.0]
          closed: true
          fill: "#090d16"
          stroke: "{{neon_border}}"
          stroke_width: 1.5
        - shape: line
          x1: -32.0
          y1: 26.0
          x2: 32.0
          y2: 26.0
          stroke: "{{neon_accent}}"
          stroke_width: 2.0
        - shape: circle
          x: -38.0
          y: 0.0
          r: 13.0
          fill: "#0f172a"
          stroke: "{{neon_accent}}"
          stroke_width: 1.2
        - shape: icon
          x: -38.0
          y: 0.0
          w: 18.0
          h: 18.0
          icon: "{{icon}}"
        - shape: text
          x: 12.0
          y: 6.0
          text: "{{node_name}}"
          size: 9.5
          color: "#f0fdf4"
        - shape: text
          x: 12.0
          y: -6.0
          text: "{{telemetry}}"
          size: 7.5
          color: "{{neon_accent}}"
        - shape: roundedrect
          x: 0.0
          y: -18.0
          w: 86.0
          h: 10.0
          radius: 2.0
          fill: "{{badge_bg}}"
        - shape: text
          x: 0.0
          y: -18.0
          text: "{{status_code}}"
          size: 6.5
          color: "{{badge_color}}"
        - shape: progress
          style: segmented
          x: 0.0
          y: -26.0
          w: 64.0
          h: 3.0
          segments: 6
          gap: 2.0
          track_color: "#0f172a"
          fill_color: "{{neon_accent}}"
"##;

pub const PRESET_THEME_CLOUD: &str = r##"
graph_defn:
  graph_attrs:
    background: "#f1f5f9"
    connection_color: "#64748b"
    text_color: "#0f172a"
    message_theme:
      shape: pill
      bg: "#1e293b"
      stroke: "#38bdf8"
      stroke_width: 1.5
      text_color: "#f8fafc"
      font_size: 13.0
  node_templates:
    - id: cloud_card
      shapes:
        - shape: roundedrect
          x: 0.0
          y: 0.0
          w: 124.0
          h: 72.0
          radius: 8.0
          fill: "#ffffff"
          stroke: "#cbd5e1"
          stroke_width: 1.5
        - shape: rect
          x: 0.0
          y: 32.0
          w: 124.0
          h: 8.0
          fill: "{{accent_color}}"
        - shape: circle
          x: -38.0
          y: 6.0
          r: 16.0
          fill: "#f8fafc"
          stroke: "#e2e8f0"
          stroke_width: 1.0
        - shape: icon
          x: -38.0
          y: 6.0
          w: 22.0
          h: 22.0
          icon: "{{icon}}"
        - shape: text
          x: 14.0
          y: 12.0
          text: "{{node_name}}"
          size: 10.0
          color: "#0f172a"
        - shape: text
          x: 14.0
          y: -2.0
          text: "{{service_type}}"
          size: 8.0
          color: "#64748b"
        - shape: roundedrect
          x: 0.0
          y: -20.0
          w: 108.0
          h: 14.0
          radius: 4.0
          fill: "{{status_bg}}"
        - shape: text
          x: 0.0
          y: -20.0
          text: "{{status_text}}"
          size: 7.5
          color: "{{status_color}}"
        - shape: progress
          style: bar
          x: 0.0
          y: -34.0
          w: 124.0
          h: 4.0
          track_color: "#f1f5f9"
          fill_color: "{{accent_color}}"
    - id: default
      shapes:
        - shape: roundedrect
          x: 0.0
          y: 0.0
          w: 124.0
          h: 72.0
          radius: 8.0
          fill: "#ffffff"
          stroke: "#cbd5e1"
          stroke_width: 1.5
        - shape: rect
          x: 0.0
          y: 32.0
          w: 124.0
          h: 8.0
          fill: "{{accent_color}}"
        - shape: circle
          x: -38.0
          y: 6.0
          r: 16.0
          fill: "#f8fafc"
          stroke: "#e2e8f0"
          stroke_width: 1.0
        - shape: icon
          x: -38.0
          y: 6.0
          w: 22.0
          h: 22.0
          icon: "{{icon}}"
        - shape: text
          x: 14.0
          y: 12.0
          text: "{{node_name}}"
          size: 10.0
          color: "#0f172a"
        - shape: text
          x: 14.0
          y: -2.0
          text: "{{service_type}}"
          size: 8.0
          color: "#64748b"
        - shape: roundedrect
          x: 0.0
          y: -20.0
          w: 108.0
          h: 14.0
          radius: 4.0
          fill: "{{status_bg}}"
        - shape: text
          x: 0.0
          y: -20.0
          text: "{{status_text}}"
          size: 7.5
          color: "{{status_color}}"
        - shape: progress
          style: bar
          x: 0.0
          y: -34.0
          w: 124.0
          h: 4.0
          track_color: "#f1f5f9"
          fill_color: "{{accent_color}}"
    - id: card
      shapes:
        - shape: roundedrect
          x: 0.0
          y: 0.0
          w: 124.0
          h: 72.0
          radius: 8.0
          fill: "#ffffff"
          stroke: "#cbd5e1"
          stroke_width: 1.5
        - shape: rect
          x: 0.0
          y: 32.0
          w: 124.0
          h: 8.0
          fill: "{{accent_color}}"
        - shape: circle
          x: -38.0
          y: 6.0
          r: 16.0
          fill: "#f8fafc"
          stroke: "#e2e8f0"
          stroke_width: 1.0
        - shape: icon
          x: -38.0
          y: 6.0
          w: 22.0
          h: 22.0
          icon: "{{icon}}"
        - shape: text
          x: 14.0
          y: 12.0
          text: "{{node_name}}"
          size: 10.0
          color: "#0f172a"
        - shape: text
          x: 14.0
          y: -2.0
          text: "{{service_type}}"
          size: 8.0
          color: "#64748b"
        - shape: roundedrect
          x: 0.0
          y: -20.0
          w: 108.0
          h: 14.0
          radius: 4.0
          fill: "{{status_bg}}"
        - shape: text
          x: 0.0
          y: -20.0
          text: "{{status_text}}"
          size: 7.5
          color: "{{status_color}}"
        - shape: progress
          style: bar
          x: 0.0
          y: -34.0
          w: 124.0
          h: 4.0
          track_color: "#f1f5f9"
          fill_color: "{{accent_color}}"
    - id: cyber_hud
      shapes:
        - shape: roundedrect
          x: 0.0
          y: 0.0
          w: 124.0
          h: 72.0
          radius: 8.0
          fill: "#ffffff"
          stroke: "#cbd5e1"
          stroke_width: 1.5
        - shape: rect
          x: 0.0
          y: 32.0
          w: 124.0
          h: 8.0
          fill: "{{accent_color}}"
        - shape: circle
          x: -38.0
          y: 6.0
          r: 16.0
          fill: "#f8fafc"
          stroke: "#e2e8f0"
          stroke_width: 1.0
        - shape: icon
          x: -38.0
          y: 6.0
          w: 22.0
          h: 22.0
          icon: "{{icon}}"
        - shape: text
          x: 14.0
          y: 12.0
          text: "{{node_name}}"
          size: 10.0
          color: "#0f172a"
        - shape: text
          x: 14.0
          y: -2.0
          text: "{{service_type}}"
          size: 8.0
          color: "#64748b"
        - shape: roundedrect
          x: 0.0
          y: -20.0
          w: 108.0
          h: 14.0
          radius: 4.0
          fill: "{{status_bg}}"
        - shape: text
          x: 0.0
          y: -20.0
          text: "{{status_text}}"
          size: 7.5
          color: "{{status_color}}"
        - shape: progress
          style: bar
          x: 0.0
          y: -34.0
          w: 124.0
          h: 4.0
          track_color: "#f1f5f9"
          fill_color: "{{accent_color}}"
    - id: rack_blade
      shapes:
        - shape: roundedrect
          x: 0.0
          y: 0.0
          w: 124.0
          h: 72.0
          radius: 8.0
          fill: "#ffffff"
          stroke: "#cbd5e1"
          stroke_width: 1.5
        - shape: rect
          x: 0.0
          y: 32.0
          w: 124.0
          h: 8.0
          fill: "{{accent_color}}"
        - shape: circle
          x: -38.0
          y: 6.0
          r: 16.0
          fill: "#f8fafc"
          stroke: "#e2e8f0"
          stroke_width: 1.0
        - shape: icon
          x: -38.0
          y: 6.0
          w: 22.0
          h: 22.0
          icon: "{{icon}}"
        - shape: text
          x: 14.0
          y: 12.0
          text: "{{node_name}}"
          size: 10.0
          color: "#0f172a"
        - shape: text
          x: 14.0
          y: -2.0
          text: "{{service_type}}"
          size: 8.0
          color: "#64748b"
        - shape: roundedrect
          x: 0.0
          y: -20.0
          w: 108.0
          h: 14.0
          radius: 4.0
          fill: "{{status_bg}}"
        - shape: text
          x: 0.0
          y: -20.0
          text: "{{status_text}}"
          size: 7.5
          color: "{{status_color}}"
        - shape: progress
          style: bar
          x: 0.0
          y: -34.0
          w: 124.0
          h: 4.0
          track_color: "#f1f5f9"
          fill_color: "{{accent_color}}"
    - id: minimal_pill
      shapes:
        - shape: roundedrect
          x: 0.0
          y: 0.0
          w: 124.0
          h: 72.0
          radius: 8.0
          fill: "#ffffff"
          stroke: "#cbd5e1"
          stroke_width: 1.5
        - shape: rect
          x: 0.0
          y: 32.0
          w: 124.0
          h: 8.0
          fill: "{{accent_color}}"
        - shape: circle
          x: -38.0
          y: 6.0
          r: 16.0
          fill: "#f8fafc"
          stroke: "#e2e8f0"
          stroke_width: 1.0
        - shape: icon
          x: -38.0
          y: 6.0
          w: 22.0
          h: 22.0
          icon: "{{icon}}"
        - shape: text
          x: 14.0
          y: 12.0
          text: "{{node_name}}"
          size: 10.0
          color: "#0f172a"
        - shape: text
          x: 14.0
          y: -2.0
          text: "{{service_type}}"
          size: 8.0
          color: "#64748b"
        - shape: roundedrect
          x: 0.0
          y: -20.0
          w: 108.0
          h: 14.0
          radius: 4.0
          fill: "{{status_bg}}"
        - shape: text
          x: 0.0
          y: -20.0
          text: "{{status_text}}"
          size: 7.5
          color: "{{status_color}}"
        - shape: progress
          style: bar
          x: 0.0
          y: -34.0
          w: 124.0
          h: 4.0
          track_color: "#f1f5f9"
          fill_color: "{{accent_color}}"
"##;

pub const PRESET_THEME_DATACENTER: &str = r##"
graph_defn:
  graph_attrs:
    background: "#18181b"
    connection_color: "#22c55e"
    text_color: "#f4f4f5"
    message_theme:
      shape: box
      bg: "#18181b"
      stroke: "#22c55e"
      stroke_width: 1.5
      text_color: "#4ade80"
      font_size: 13.0
  node_templates:
    - id: rack_blade
      shapes:
        - shape: roundedrect
          x: 0.0
          y: 0.0
          w: 136.0
          h: 44.0
          radius: 4.0
          fill: "#1e293b"
          stroke: "#475569"
          stroke_width: 1.5
        - shape: rect
          x: -64.0
          y: 0.0
          w: 8.0
          h: 44.0
          fill: "#0f172a"
          stroke: "#334155"
          stroke_width: 1.0
        - shape: rect
          x: 64.0
          y: 0.0
          w: 8.0
          h: 44.0
          fill: "#0f172a"
          stroke: "#334155"
          stroke_width: 1.0
        - shape: circle
          x: -64.0
          y: 13.0
          r: 2.2
          fill: "#94a3b8"
        - shape: circle
          x: -64.0
          y: -13.0
          r: 2.2
          fill: "#94a3b8"
        - shape: circle
          x: 64.0
          y: 13.0
          r: 2.2
          fill: "#94a3b8"
        - shape: circle
          x: 64.0
          y: -13.0
          r: 2.2
          fill: "#94a3b8"
        - shape: icon
          x: -40.0
          y: 0.0
          w: 22.0
          h: 22.0
          icon: "{{icon}}"
        - shape: text
          x: 3.0
          y: 6.0
          text: "{{node_name}}"
          size: 9.5
          color: "#f8fafc"
        - shape: text
          x: 3.0
          y: -8.0
          text: "{{unit_id}}"
          size: 7.5
          color: "#94a3b8"
        - shape: circle
          x: 46.0
          y: 10.0
          r: 2.5
          fill: "{{led_pwr}}"
        - shape: circle
          x: 46.0
          y: 0.0
          r: 2.5
          fill: "{{led_net}}"
        - shape: circle
          x: 46.0
          y: -10.0
          r: 2.5
          fill: "{{led_act}}"
        - shape: progress
          style: segmented
          x: 0.0
          y: -19.0
          w: 110.0
          h: 3.0
          segments: 10
          gap: 2.0
          track_color: "#0f172a"
          fill_color: "#22c55e"
    - id: rack_chassis
      shapes:
        - shape: roundedrect
          x: 0.0
          y: 0.0
          w: 136.0
          h: 44.0
          radius: 4.0
          fill: "#1e293b"
          stroke: "#475569"
          stroke_width: 1.5
        - shape: rect
          x: -64.0
          y: 0.0
          w: 8.0
          h: 44.0
          fill: "#0f172a"
          stroke: "#334155"
          stroke_width: 1.0
        - shape: rect
          x: 64.0
          y: 0.0
          w: 8.0
          h: 44.0
          fill: "#0f172a"
          stroke: "#334155"
          stroke_width: 1.0
        - shape: circle
          x: -64.0
          y: 13.0
          r: 2.2
          fill: "#94a3b8"
        - shape: circle
          x: -64.0
          y: -13.0
          r: 2.2
          fill: "#94a3b8"
        - shape: circle
          x: 64.0
          y: 13.0
          r: 2.2
          fill: "#94a3b8"
        - shape: circle
          x: 64.0
          y: -13.0
          r: 2.2
          fill: "#94a3b8"
        - shape: icon
          x: -40.0
          y: 0.0
          w: 22.0
          h: 22.0
          icon: "{{icon}}"
        - shape: text
          x: 3.0
          y: 6.0
          text: "{{node_name}}"
          size: 9.5
          color: "#f8fafc"
        - shape: text
          x: 3.0
          y: -8.0
          text: "{{unit_id}}"
          size: 7.5
          color: "#94a3b8"
        - shape: circle
          x: 46.0
          y: 10.0
          r: 2.5
          fill: "{{led_pwr}}"
        - shape: circle
          x: 46.0
          y: 0.0
          r: 2.5
          fill: "{{led_net}}"
        - shape: circle
          x: 46.0
          y: -10.0
          r: 2.5
          fill: "{{led_act}}"
        - shape: progress
          style: segmented
          x: 0.0
          y: -19.0
          w: 110.0
          h: 3.0
          segments: 10
          gap: 2.0
          track_color: "#0f172a"
          fill_color: "#22c55e"
    - id: default
      shapes:
        - shape: roundedrect
          x: 0.0
          y: 0.0
          w: 136.0
          h: 44.0
          radius: 4.0
          fill: "#1e293b"
          stroke: "#475569"
          stroke_width: 1.5
        - shape: rect
          x: -64.0
          y: 0.0
          w: 8.0
          h: 44.0
          fill: "#0f172a"
          stroke: "#334155"
          stroke_width: 1.0
        - shape: rect
          x: 64.0
          y: 0.0
          w: 8.0
          h: 44.0
          fill: "#0f172a"
          stroke: "#334155"
          stroke_width: 1.0
        - shape: circle
          x: -64.0
          y: 13.0
          r: 2.2
          fill: "#94a3b8"
        - shape: circle
          x: -64.0
          y: -13.0
          r: 2.2
          fill: "#94a3b8"
        - shape: circle
          x: 64.0
          y: 13.0
          r: 2.2
          fill: "#94a3b8"
        - shape: circle
          x: 64.0
          y: -13.0
          r: 2.2
          fill: "#94a3b8"
        - shape: icon
          x: -40.0
          y: 0.0
          w: 22.0
          h: 22.0
          icon: "{{icon}}"
        - shape: text
          x: 3.0
          y: 6.0
          text: "{{node_name}}"
          size: 9.5
          color: "#f8fafc"
        - shape: text
          x: 3.0
          y: -8.0
          text: "{{unit_id}}"
          size: 7.5
          color: "#94a3b8"
        - shape: circle
          x: 46.0
          y: 10.0
          r: 2.5
          fill: "{{led_pwr}}"
        - shape: circle
          x: 46.0
          y: 0.0
          r: 2.5
          fill: "{{led_net}}"
        - shape: circle
          x: 46.0
          y: -10.0
          r: 2.5
          fill: "{{led_act}}"
        - shape: progress
          style: segmented
          x: 0.0
          y: -19.0
          w: 110.0
          h: 3.0
          segments: 10
          gap: 2.0
          track_color: "#0f172a"
          fill_color: "#22c55e"
    - id: card
      shapes:
        - shape: roundedrect
          x: 0.0
          y: 0.0
          w: 136.0
          h: 44.0
          radius: 4.0
          fill: "#1e293b"
          stroke: "#475569"
          stroke_width: 1.5
        - shape: rect
          x: -64.0
          y: 0.0
          w: 8.0
          h: 44.0
          fill: "#0f172a"
          stroke: "#334155"
          stroke_width: 1.0
        - shape: rect
          x: 64.0
          y: 0.0
          w: 8.0
          h: 44.0
          fill: "#0f172a"
          stroke: "#334155"
          stroke_width: 1.0
        - shape: circle
          x: -64.0
          y: 13.0
          r: 2.2
          fill: "#94a3b8"
        - shape: circle
          x: -64.0
          y: -13.0
          r: 2.2
          fill: "#94a3b8"
        - shape: circle
          x: 64.0
          y: 13.0
          r: 2.2
          fill: "#94a3b8"
        - shape: circle
          x: 64.0
          y: -13.0
          r: 2.2
          fill: "#94a3b8"
        - shape: icon
          x: -40.0
          y: 0.0
          w: 22.0
          h: 22.0
          icon: "{{icon}}"
        - shape: text
          x: 3.0
          y: 6.0
          text: "{{node_name}}"
          size: 9.5
          color: "#f8fafc"
        - shape: text
          x: 3.0
          y: -8.0
          text: "{{unit_id}}"
          size: 7.5
          color: "#94a3b8"
        - shape: circle
          x: 46.0
          y: 10.0
          r: 2.5
          fill: "{{led_pwr}}"
        - shape: circle
          x: 46.0
          y: 0.0
          r: 2.5
          fill: "{{led_net}}"
        - shape: circle
          x: 46.0
          y: -10.0
          r: 2.5
          fill: "{{led_act}}"
        - shape: progress
          style: segmented
          x: 0.0
          y: -19.0
          w: 110.0
          h: 3.0
          segments: 10
          gap: 2.0
          track_color: "#0f172a"
          fill_color: "#22c55e"
    - id: cloud_card
      shapes:
        - shape: roundedrect
          x: 0.0
          y: 0.0
          w: 136.0
          h: 44.0
          radius: 4.0
          fill: "#1e293b"
          stroke: "#475569"
          stroke_width: 1.5
        - shape: rect
          x: -64.0
          y: 0.0
          w: 8.0
          h: 44.0
          fill: "#0f172a"
          stroke: "#334155"
          stroke_width: 1.0
        - shape: rect
          x: 64.0
          y: 0.0
          w: 8.0
          h: 44.0
          fill: "#0f172a"
          stroke: "#334155"
          stroke_width: 1.0
        - shape: circle
          x: -64.0
          y: 13.0
          r: 2.2
          fill: "#94a3b8"
        - shape: circle
          x: -64.0
          y: -13.0
          r: 2.2
          fill: "#94a3b8"
        - shape: circle
          x: 64.0
          y: 13.0
          r: 2.2
          fill: "#94a3b8"
        - shape: circle
          x: 64.0
          y: -13.0
          r: 2.2
          fill: "#94a3b8"
        - shape: icon
          x: -40.0
          y: 0.0
          w: 22.0
          h: 22.0
          icon: "{{icon}}"
        - shape: text
          x: 3.0
          y: 6.0
          text: "{{node_name}}"
          size: 9.5
          color: "#f8fafc"
        - shape: text
          x: 3.0
          y: -8.0
          text: "{{unit_id}}"
          size: 7.5
          color: "#94a3b8"
        - shape: circle
          x: 46.0
          y: 10.0
          r: 2.5
          fill: "{{led_pwr}}"
        - shape: circle
          x: 46.0
          y: 0.0
          r: 2.5
          fill: "{{led_net}}"
        - shape: circle
          x: 46.0
          y: -10.0
          r: 2.5
          fill: "{{led_act}}"
        - shape: progress
          style: segmented
          x: 0.0
          y: -19.0
          w: 110.0
          h: 3.0
          segments: 10
          gap: 2.0
          track_color: "#0f172a"
          fill_color: "#22c55e"
    - id: cyber_hud
      shapes:
        - shape: roundedrect
          x: 0.0
          y: 0.0
          w: 136.0
          h: 44.0
          radius: 4.0
          fill: "#1e293b"
          stroke: "#475569"
          stroke_width: 1.5
        - shape: rect
          x: -64.0
          y: 0.0
          w: 8.0
          h: 44.0
          fill: "#0f172a"
          stroke: "#334155"
          stroke_width: 1.0
        - shape: rect
          x: 64.0
          y: 0.0
          w: 8.0
          h: 44.0
          fill: "#0f172a"
          stroke: "#334155"
          stroke_width: 1.0
        - shape: circle
          x: -64.0
          y: 13.0
          r: 2.2
          fill: "#94a3b8"
        - shape: circle
          x: -64.0
          y: -13.0
          r: 2.2
          fill: "#94a3b8"
        - shape: circle
          x: 64.0
          y: 13.0
          r: 2.2
          fill: "#94a3b8"
        - shape: circle
          x: 64.0
          y: -13.0
          r: 2.2
          fill: "#94a3b8"
        - shape: icon
          x: -40.0
          y: 0.0
          w: 22.0
          h: 22.0
          icon: "{{icon}}"
        - shape: text
          x: 3.0
          y: 6.0
          text: "{{node_name}}"
          size: 9.5
          color: "#f8fafc"
        - shape: text
          x: 3.0
          y: -8.0
          text: "{{unit_id}}"
          size: 7.5
          color: "#94a3b8"
        - shape: circle
          x: 46.0
          y: 10.0
          r: 2.5
          fill: "{{led_pwr}}"
        - shape: circle
          x: 46.0
          y: 0.0
          r: 2.5
          fill: "{{led_net}}"
        - shape: circle
          x: 46.0
          y: -10.0
          r: 2.5
          fill: "{{led_act}}"
        - shape: progress
          style: segmented
          x: 0.0
          y: -19.0
          w: 110.0
          h: 3.0
          segments: 10
          gap: 2.0
          track_color: "#0f172a"
          fill_color: "#22c55e"
    - id: minimal_pill
      shapes:
        - shape: roundedrect
          x: 0.0
          y: 0.0
          w: 136.0
          h: 44.0
          radius: 4.0
          fill: "#1e293b"
          stroke: "#475569"
          stroke_width: 1.5
        - shape: rect
          x: -64.0
          y: 0.0
          w: 8.0
          h: 44.0
          fill: "#0f172a"
          stroke: "#334155"
          stroke_width: 1.0
        - shape: rect
          x: 64.0
          y: 0.0
          w: 8.0
          h: 44.0
          fill: "#0f172a"
          stroke: "#334155"
          stroke_width: 1.0
        - shape: circle
          x: -64.0
          y: 13.0
          r: 2.2
          fill: "#94a3b8"
        - shape: circle
          x: -64.0
          y: -13.0
          r: 2.2
          fill: "#94a3b8"
        - shape: circle
          x: 64.0
          y: 13.0
          r: 2.2
          fill: "#94a3b8"
        - shape: circle
          x: 64.0
          y: -13.0
          r: 2.2
          fill: "#94a3b8"
        - shape: icon
          x: -40.0
          y: 0.0
          w: 22.0
          h: 22.0
          icon: "{{icon}}"
        - shape: text
          x: 3.0
          y: 6.0
          text: "{{node_name}}"
          size: 9.5
          color: "#f8fafc"
        - shape: text
          x: 3.0
          y: -8.0
          text: "{{unit_id}}"
          size: 7.5
          color: "#94a3b8"
        - shape: circle
          x: 46.0
          y: 10.0
          r: 2.5
          fill: "{{led_pwr}}"
        - shape: circle
          x: 46.0
          y: 0.0
          r: 2.5
          fill: "{{led_net}}"
        - shape: circle
          x: 46.0
          y: -10.0
          r: 2.5
          fill: "{{led_act}}"
        - shape: progress
          style: segmented
          x: 0.0
          y: -19.0
          w: 110.0
          h: 3.0
          segments: 10
          gap: 2.0
          track_color: "#0f172a"
          fill_color: "#22c55e"
"##;

pub const PRESET_THEME_MINIMAL: &str = r##"
graph_defn:
  graph_attrs:
    background: "#f8fafc"
    connection_color: "#3b82f6"
    text_color: "#0f172a"
    message_theme:
      shape: pill
      bg: "#ffffff"
      stroke: "#94a3b8"
      stroke_width: 1.5
      text_color: "#1e293b"
      font_size: 13.0
  node_templates:
    - id: minimal_pill
      shapes:
        - shape: roundedrect
          x: 0.0
          y: 0.0
          w: 114.0
          h: 32.0
          radius: 16.0
          fill: "#ffffff"
          stroke: "#cbd5e1"
          stroke_width: 1.5
        - shape: circle
          x: -41.0
          y: 0.0
          r: 12.0
          fill: "{{disc_color}}"
        - shape: icon
          x: -41.0
          y: 0.0
          w: 16.0
          h: 16.0
          icon: "{{icon}}"
        - shape: progress
          style: ring
          x: -41.0
          y: 0.0
          r: 14.5
          thickness: 2.0
          track_color: "#e2e8f0"
          fill_color: "{{tag_color}}"
          start_angle: 90.0
          clockwise: true
        - shape: line
          x1: -24.0
          y1: 8.0
          x2: -24.0
          y2: -8.0
          stroke: "#e2e8f0"
          stroke_width: 1.0
        - shape: text
          x: 16.0
          y: 4.0
          text: "{{node_name}}"
          size: 9.0
          color: "#1e293b"
        - shape: text
          x: 16.0
          y: -6.0
          text: "{{role_tag}}"
          size: 7.0
          color: "{{tag_color}}"
    - id: capsule_pill
      shapes:
        - shape: roundedrect
          x: 0.0
          y: 0.0
          w: 114.0
          h: 32.0
          radius: 16.0
          fill: "#ffffff"
          stroke: "#cbd5e1"
          stroke_width: 1.5
        - shape: circle
          x: -41.0
          y: 0.0
          r: 12.0
          fill: "{{disc_color}}"
        - shape: icon
          x: -41.0
          y: 0.0
          w: 16.0
          h: 16.0
          icon: "{{icon}}"
        - shape: progress
          style: ring
          x: -41.0
          y: 0.0
          r: 14.5
          thickness: 2.0
          track_color: "#e2e8f0"
          fill_color: "{{tag_color}}"
          start_angle: 90.0
          clockwise: true
        - shape: line
          x1: -24.0
          y1: 8.0
          x2: -24.0
          y2: -8.0
          stroke: "#e2e8f0"
          stroke_width: 1.0
        - shape: text
          x: 16.0
          y: 4.0
          text: "{{node_name}}"
          size: 9.0
          color: "#1e293b"
        - shape: text
          x: 16.0
          y: -6.0
          text: "{{role_tag}}"
          size: 7.0
          color: "{{tag_color}}"
    - id: default
      shapes:
        - shape: roundedrect
          x: 0.0
          y: 0.0
          w: 114.0
          h: 32.0
          radius: 16.0
          fill: "#ffffff"
          stroke: "#cbd5e1"
          stroke_width: 1.5
        - shape: circle
          x: -41.0
          y: 0.0
          r: 12.0
          fill: "{{disc_color}}"
        - shape: icon
          x: -41.0
          y: 0.0
          w: 16.0
          h: 16.0
          icon: "{{icon}}"
        - shape: progress
          style: ring
          x: -41.0
          y: 0.0
          r: 14.5
          thickness: 2.0
          track_color: "#e2e8f0"
          fill_color: "{{tag_color}}"
          start_angle: 90.0
          clockwise: true
        - shape: line
          x1: -24.0
          y1: 8.0
          x2: -24.0
          y2: -8.0
          stroke: "#e2e8f0"
          stroke_width: 1.0
        - shape: text
          x: 16.0
          y: 4.0
          text: "{{node_name}}"
          size: 9.0
          color: "#1e293b"
        - shape: text
          x: 16.0
          y: -6.0
          text: "{{role_tag}}"
          size: 7.0
          color: "{{tag_color}}"
    - id: card
      shapes:
        - shape: roundedrect
          x: 0.0
          y: 0.0
          w: 114.0
          h: 32.0
          radius: 16.0
          fill: "#ffffff"
          stroke: "#cbd5e1"
          stroke_width: 1.5
        - shape: circle
          x: -41.0
          y: 0.0
          r: 12.0
          fill: "{{disc_color}}"
        - shape: icon
          x: -41.0
          y: 0.0
          w: 16.0
          h: 16.0
          icon: "{{icon}}"
        - shape: progress
          style: ring
          x: -41.0
          y: 0.0
          r: 14.5
          thickness: 2.0
          track_color: "#e2e8f0"
          fill_color: "{{tag_color}}"
          start_angle: 90.0
          clockwise: true
        - shape: line
          x1: -24.0
          y1: 8.0
          x2: -24.0
          y2: -8.0
          stroke: "#e2e8f0"
          stroke_width: 1.0
        - shape: text
          x: 16.0
          y: 4.0
          text: "{{node_name}}"
          size: 9.0
          color: "#1e293b"
        - shape: text
          x: 16.0
          y: -6.0
          text: "{{role_tag}}"
          size: 7.0
          color: "{{tag_color}}"
    - id: cloud_card
      shapes:
        - shape: roundedrect
          x: 0.0
          y: 0.0
          w: 114.0
          h: 32.0
          radius: 16.0
          fill: "#ffffff"
          stroke: "#cbd5e1"
          stroke_width: 1.5
        - shape: circle
          x: -41.0
          y: 0.0
          r: 12.0
          fill: "{{disc_color}}"
        - shape: icon
          x: -41.0
          y: 0.0
          w: 16.0
          h: 16.0
          icon: "{{icon}}"
        - shape: progress
          style: ring
          x: -41.0
          y: 0.0
          r: 14.5
          thickness: 2.0
          track_color: "#e2e8f0"
          fill_color: "{{tag_color}}"
          start_angle: 90.0
          clockwise: true
        - shape: line
          x1: -24.0
          y1: 8.0
          x2: -24.0
          y2: -8.0
          stroke: "#e2e8f0"
          stroke_width: 1.0
        - shape: text
          x: 16.0
          y: 4.0
          text: "{{node_name}}"
          size: 9.0
          color: "#1e293b"
        - shape: text
          x: 16.0
          y: -6.0
          text: "{{role_tag}}"
          size: 7.0
          color: "{{tag_color}}"
    - id: cyber_hud
      shapes:
        - shape: roundedrect
          x: 0.0
          y: 0.0
          w: 114.0
          h: 32.0
          radius: 16.0
          fill: "#ffffff"
          stroke: "#cbd5e1"
          stroke_width: 1.5
        - shape: circle
          x: -41.0
          y: 0.0
          r: 12.0
          fill: "{{disc_color}}"
        - shape: icon
          x: -41.0
          y: 0.0
          w: 16.0
          h: 16.0
          icon: "{{icon}}"
        - shape: progress
          style: ring
          x: -41.0
          y: 0.0
          r: 14.5
          thickness: 2.0
          track_color: "#e2e8f0"
          fill_color: "{{tag_color}}"
          start_angle: 90.0
          clockwise: true
        - shape: line
          x1: -24.0
          y1: 8.0
          x2: -24.0
          y2: -8.0
          stroke: "#e2e8f0"
          stroke_width: 1.0
        - shape: text
          x: 16.0
          y: 4.0
          text: "{{node_name}}"
          size: 9.0
          color: "#1e293b"
        - shape: text
          x: 16.0
          y: -6.0
          text: "{{role_tag}}"
          size: 7.0
          color: "{{tag_color}}"
    - id: rack_blade
      shapes:
        - shape: roundedrect
          x: 0.0
          y: 0.0
          w: 114.0
          h: 32.0
          radius: 16.0
          fill: "#ffffff"
          stroke: "#cbd5e1"
          stroke_width: 1.5
        - shape: circle
          x: -41.0
          y: 0.0
          r: 12.0
          fill: "{{disc_color}}"
        - shape: icon
          x: -41.0
          y: 0.0
          w: 16.0
          h: 16.0
          icon: "{{icon}}"
        - shape: progress
          style: ring
          x: -41.0
          y: 0.0
          r: 14.5
          thickness: 2.0
          track_color: "#e2e8f0"
          fill_color: "{{tag_color}}"
          start_angle: 90.0
          clockwise: true
        - shape: line
          x1: -24.0
          y1: 8.0
          x2: -24.0
          y2: -8.0
          stroke: "#e2e8f0"
          stroke_width: 1.0
        - shape: text
          x: 16.0
          y: 4.0
          text: "{{node_name}}"
          size: 9.0
          color: "#1e293b"
        - shape: text
          x: 16.0
          y: -6.0
          text: "{{role_tag}}"
          size: 7.0
          color: "{{tag_color}}"
"##;

pub const PRESET_STDLIB_LOAD_BALANCER: &str = r##"
graph_defn:
  node_types:
    - id: round_robin_lb
      attrs:
        ticks: 1
      fn: |
        fn on_init() {
          state.idx = 0;
        }
        fn on_timer() {}
        fn on_message(msg) {
          if links.len() == 0 { return; }
          let target = links[state.idx % links.len()];
          state.idx += 1;
          send(target, msg);
        }
    - id: weighted_lb
      attrs:
        ticks: 1
      fn: |
        fn on_init() {
          state.count = 0;
        }
        fn on_timer() {}
        fn on_message(msg) {
          if links.len() == 0 { return; }
          let target = links[state.count % links.len()];
          state.count += 1;
          send(target, msg);
        }
"##;

pub const PRESET_STDLIB_CIRCUIT_BREAKER: &str = r##"
graph_defn:
  node_types:
    - id: circuit_breaker
      attrs:
        ticks: 1
      fn: |
        fn on_init() {
          state.status = "CLOSED";
          state.failures = 0;
          state.threshold = 3;
        }
        fn on_timer() {}
        fn on_message(msg) {
          if state.status == "OPEN" {
            log("Circuit breaker OPEN - dropping message");
            return;
          }
          if links.len() > 0 {
            send(links[0], msg);
          }
        }
"##;

pub const PRESET_STDLIB_CACHE: &str = r##"
graph_defn:
  node_types:
    - id: lru_cache
      attrs:
        ticks: 1
      fn: |
        fn on_init() {
          state.store = #{};
        }
        fn on_timer() {}
        fn on_message(msg) {
          if state.store.contains(msg) {
            log("Cache HIT for: " + msg);
          } else {
            log("Cache MISS for: " + msg);
            state.store[msg] = true;
            if links.len() > 0 {
              send(links[0], msg);
            }
          }
        }
"##;

/// Looks up a built-in preset by name/shorthand.
pub fn get_builtin_preset(name: &str) -> Option<&'static str> {
    let normalized = name.trim().to_lowercase();
    match normalized.as_str() {
        "theme:cyberpunk" | "cyberpunk" => Some(PRESET_THEME_CYBERPUNK),
        "theme:cloud" | "theme:cloud_cards" | "cloud" | "cloud_cards" => Some(PRESET_THEME_CLOUD),
        "theme:datacenter" | "theme:rack" | "datacenter" | "rack" => Some(PRESET_THEME_DATACENTER),
        "theme:minimal" | "theme:capsule" | "minimal" | "capsule" => Some(PRESET_THEME_MINIMAL),
        "stdlib:load_balancer" | "load_balancer" | "stdlib:lb" | "lb" => {
            Some(PRESET_STDLIB_LOAD_BALANCER)
        }
        "stdlib:circuit_breaker" | "circuit_breaker" => Some(PRESET_STDLIB_CIRCUIT_BREAKER),
        "stdlib:cache" | "cache" | "stdlib:lru_cache" | "lru_cache" => Some(PRESET_STDLIB_CACHE),
        _ => None,
    }
}

/// Scans raw YAML string for remote HTTP/HTTPS import URLs.
pub fn scan_import_urls(raw_yaml: &str) -> Vec<String> {
    let mut urls = Vec::new();

    // Fast heuristic line scanner to avoid failing if incomplete YAML is being edited
    for line in raw_yaml.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("from:") || trimmed.starts_with("- from:") {
            if let Some(val_idx) = trimmed.find("from:") {
                let after = trimmed[val_idx + 5..].trim();
                let url = after
                    .trim_matches(|c| c == '\'' || c == '"' || c == ' ')
                    .to_string();
                if url.starts_with("http://") || url.starts_with("https://") {
                    if !urls.contains(&url) {
                        urls.push(url);
                    }
                }
            }
        }
    }

    // Also attempt parsing as File to catch full structured imports
    if let Ok(file) = serde_yaml::from_str::<File>(raw_yaml) {
        for imp in file
            .imports
            .iter()
            .chain(file.graph_defn.imports.iter())
        {
            if imp.from.starts_with("http://") || imp.from.starts_with("https://") {
                if !urls.contains(&imp.from) {
                    urls.push(imp.from.clone());
                }
            }
        }
    }

    urls
}

/// Merges an imported `GraphDefinition` into a base `GraphDefinition`.
/// Local elements always take precedence over imported ones.
pub fn merge_graph_definitions(
    base: &mut GraphDefinition,
    imported: GraphDefinition,
    filter: Option<&[String]>,
    alias: Option<&str>,
) {
    // 1. Merge graph_attrs if base still has default values
    let default_attrs = crate::parser::graphv2::GraphAttrs::default();
    if base.graph_attrs.background == default_attrs.background
        && imported.graph_attrs.background != default_attrs.background
    {
        base.graph_attrs.background = imported.graph_attrs.background;
    }
    if base.graph_attrs.connection_color == default_attrs.connection_color
        && imported.graph_attrs.connection_color != default_attrs.connection_color
    {
        base.graph_attrs.connection_color = imported.graph_attrs.connection_color;
    }
    if base.graph_attrs.text_color == default_attrs.text_color
        && imported.graph_attrs.text_color != default_attrs.text_color
    {
        base.graph_attrs.text_color = imported.graph_attrs.text_color;
    }
    if base.graph_attrs.title.is_empty() && !imported.graph_attrs.title.is_empty() {
        base.graph_attrs.title = imported.graph_attrs.title;
    }
    if base.graph_attrs.message_theme.is_none() && imported.graph_attrs.message_theme.is_some() {
        base.graph_attrs.message_theme = imported.graph_attrs.message_theme;
    }

    // 2. Merge icons (deduplicated by id)
    for icon in imported.icons {
        if !base.icons.iter().any(|i| i.id == icon.id) {
            base.icons.push(icon);
        }
    }

    // 3. Merge node_templates (deduplicated by id)
    for tmpl in imported.node_templates {
        if !base.node_templates.iter().any(|t| t.id == tmpl.id) {
            base.node_templates.push(tmpl);
        }
    }

    // 4. Merge node_types (respecting filter & local override precedence)
    for mut nt in imported.node_types {
        if let Some(flt) = filter {
            if !flt.iter().any(|f| f == &nt.id) {
                continue;
            }
        }
        if let Some(al) = alias {
            if filter.map_or(true, |f| f.len() == 1) {
                nt.id = al.to_string();
            } else {
                nt.id = format!("{}_{}", al, nt.id);
            }
        }
        if let Some(existing) = base.node_types.iter_mut().find(|n| n.id == nt.id) {
            if existing.func.as_ref().map_or(true, |s| s.trim().is_empty()) {
                existing.func = nt.func;
            }
        } else {
            base.node_types.push(nt);
        }
    }

    // 5. Merge graph connections if base doesn't have an instance with the same name
    for conn in imported.graph {
        if !base.graph.iter().any(|c| c.name == conn.name) {
            base.graph.push(conn);
        }
    }
}

/// Resolves all imports (both built-in presets and external URLs) for a given raw YAML string.
pub fn resolve_file_imports(
    raw_yaml: &str,
    external_sources: &HashMap<String, String>,
) -> Result<File, serde_yaml::Error> {
    use serde::de::Error as SerdeError;
    let mut file: File = serde_yaml::from_str(raw_yaml)?;

    // Collect all imports: theme shorthand + top-level imports + graph_defn.imports
    let mut all_imports = Vec::new();

    // 1. Theme shorthand
    let theme_opt = file.theme.clone().or_else(|| file.graph_defn.theme.clone());
    if let Some(theme_name) = theme_opt {
        all_imports.push(ImportDef {
            from: theme_name,
            import: None,
            r#as: None,
        });
    }

    // 2. Explicit imports
    for imp in file.imports.clone() {
        all_imports.push(imp);
    }
    for imp in file.graph_defn.imports.clone() {
        all_imports.push(imp);
    }

    // Base definition starts with local file's graph_defn
    let mut merged_graph_defn = file.graph_defn.clone();

    // Process each import in sequence
    for imp in all_imports {
        let source_yaml: String = if let Some(preset_str) = get_builtin_preset(&imp.from) {
            preset_str.to_string()
        } else if let Some(fetched) = external_sources.get(&imp.from) {
            fetched.clone()
        } else if imp.from.starts_with("http://") || imp.from.starts_with("https://") {
            return Err(serde_yaml::Error::custom(format!(
                "Remote import '{}' is not yet loaded or failed to fetch",
                imp.from
            )));
        } else {
            return Err(serde_yaml::Error::custom(format!(
                "Unknown import or preset '{}'",
                imp.from
            )));
        };

        // Directly parse imported YAML
        let imported_file: File = serde_yaml::from_str(&source_yaml)?;
        let filter_slice = imp.import.as_deref();
        merge_graph_definitions(
            &mut merged_graph_defn,
            imported_file.graph_defn,
            filter_slice,
            imp.r#as.as_deref(),
        );
    }

    file.graph_defn = merged_graph_defn;
    Ok(file)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::graphv2::{parse_graph2, parse_graph2_with_sources, MessageBubbleShape};

    #[test]
    fn test_builtin_theme_cyberpunk() {
        let yaml = r#"
theme: cyberpunk
graph_defn:
  graph:
    - name: node_a
      node_type: client
      links: []
  node_types:
    - id: client
      attrs:
        ticks: 2
"#.to_string();

        let file = parse_graph2(&yaml).expect("Should parse with cyberpunk theme");
        assert_eq!(
            file.graph_defn.graph_attrs.message_theme.as_ref().unwrap().shape,
            MessageBubbleShape::Chamfered
        );
        assert!(file.graph_defn.node_templates.iter().any(|t| t.id == "cyber_hud"));
        assert_eq!(file.graph_defn.node_instances.len(), 1);
    }

    #[test]
    fn test_theme_switching_template_cross_compatibility() {
        let yaml_cyber = r#"
theme: cyberpunk
graph_defn:
  graph:
    - name: n1
      node_type: t1
      links: []
  node_types:
    - id: t1
      attrs:
        template_ref: cyber_hud
"#.to_string();
        assert!(parse_graph2(&yaml_cyber).is_ok());

        let yaml_cloud = r#"
theme: cloud
graph_defn:
  graph:
    - name: n1
      node_type: t1
      links: []
  node_types:
    - id: t1
      attrs:
        template_ref: cyber_hud
"#.to_string();
        assert!(parse_graph2(&yaml_cloud).is_ok());

        let yaml_datacenter = r#"
theme: datacenter
graph_defn:
  graph:
    - name: n1
      node_type: t1
      links: []
  node_types:
    - id: t1
      attrs:
        template_ref: cyber_hud
"#.to_string();
        assert!(parse_graph2(&yaml_datacenter).is_ok());

        let yaml_minimal = r#"
theme: minimal
graph_defn:
  graph:
    - name: n1
      node_type: t1
      links: []
  node_types:
    - id: t1
      attrs:
        template_ref: cyber_hud
"#.to_string();
        assert!(parse_graph2(&yaml_minimal).is_ok());
    }

    #[test]
    fn test_builtin_stdlib_load_balancer() {
        let yaml = r#"
imports:
  - from: "stdlib:load_balancer"
graph_defn:
  graph:
    - name: lb
      node_type: round_robin_lb
      links: ["srv1"]
    - name: srv1
      node_type: server
      links: []
  node_types:
    - id: server
      attrs:
        ticks: 1
"#.to_string();

        let file = parse_graph2(&yaml).expect("Should parse with stdlib load balancer");
        assert_eq!(file.graph_defn.node_instances.len(), 2);
        assert!(file.graph_defn.node_types.iter().any(|t| t.id == "round_robin_lb"));
        assert!(file.graph_defn.node_types.iter().any(|t| t.id == "weighted_lb"));
    }

    #[test]
    fn test_selective_import_and_aliasing() {
        let yaml = r#"
imports:
  - from: "stdlib:load_balancer"
    import: ["round_robin_lb"]
    as: "my_lb"
graph_defn:
  graph:
    - name: lb
      node_type: my_lb
      links: []
"#.to_string();

        let file = parse_graph2(&yaml).expect("Should parse with aliased selective import");
        assert!(file.graph_defn.node_types.iter().any(|t| t.id == "my_lb"));
        assert!(!file.graph_defn.node_types.iter().any(|t| t.id == "weighted_lb"));
        assert_eq!(file.graph_defn.node_instances[0].node_data.id, "my_lb");
    }

    #[test]
    fn test_remote_url_scanning_and_resolution() {
        let yaml = r#"
imports:
  - from: "https://example.com/custom_nodes.yml"
graph_defn:
  graph:
    - name: worker
      node_type: remote_worker
      links: []
"#.to_string();

        let urls = scan_import_urls(&yaml);
        assert_eq!(urls, vec!["https://example.com/custom_nodes.yml".to_string()]);

        let remote_content = r#"
graph_defn:
  node_types:
    - id: remote_worker
      attrs:
        ticks: 5
      fn: |
        fn on_init() { log("Remote worker ready"); }
"#;
        let mut sources = HashMap::new();
        sources.insert("https://example.com/custom_nodes.yml".to_string(), remote_content.to_string());

        let file = parse_graph2_with_sources(&yaml, &sources).expect("Should parse with remote sources");
        assert_eq!(file.graph_defn.node_instances.len(), 1);
        assert_eq!(file.graph_defn.node_instances[0].node_data.id, "remote_worker");
    }

    #[test]
    fn test_local_override_precedence() {
        let yaml = r#"
theme: cyberpunk
graph_defn:
  graph_attrs:
    title: "My Custom Cyberpunk Graph"
  node_types:
    - id: round_robin_lb
      attrs:
        ticks: 99
imports:
  - from: "stdlib:load_balancer"
"#.to_string();

        let file = parse_graph2(&yaml).expect("Should parse with override");
        assert_eq!(file.graph_defn.graph_attrs.title, "My Custom Cyberpunk Graph");
        let lb_type = file.graph_defn.node_types.iter().find(|t| t.id == "round_robin_lb").unwrap();
        assert_eq!(lb_type.attrs.ticks, crate::parser::graphv2::Ticks::Fixed(99));
        assert!(lb_type.func.is_some(), "Should inherit func from stdlib:load_balancer");
    }

    #[test]
    fn test_all_builtin_presets_validity() {
        let presets = [
            "theme:cyberpunk",
            "theme:cloud",
            "theme:datacenter",
            "theme:minimal",
            "stdlib:load_balancer",
            "stdlib:circuit_breaker",
            "stdlib:cache",
        ];

        for preset in presets {
            let yaml = format!("imports:\n  - from: \"{}\"\n", preset);
            let file = parse_graph2(&yaml).unwrap_or_else(|e| panic!("Preset '{}' failed to parse: {}", preset, e));
            let _ = file;
        }
    }
}
