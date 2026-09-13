# 3.3 Cyberpunk Neon HUD

Who says distributed systems diagrams have to look like plain office flowcharts? Using `polygon` geometry, glowing accent lines, and dark high-contrast color palettes, you can create a futuristic **Cyberpunk Neon HUD** theme.

### Chamfered Polygon Framing

Rather than standard rounded rectangles, sci-fi HUDs often feature **chamfered corners** (cut-off angled edges). We can build this using a closed `polygon` shape:

```yaml
node_templates:
  - id: cyber_hud
    params: [node_name, icon, neon_border, neon_accent, telemetry, badge_bg, badge_color, status_code]
    shapes:
      # Chamfered dark background plate
      - shape: polygon
        points:
          - [-48, 32]
          - [48, 32]
          - [60, 20]
          - [60, -20]
          - [48, -32]
          - [-48, -32]
          - [-60, -20]
          - [-60, 20]
        closed: true
        fill: "#090d16"
        stroke: "{{neon_border}}"
        stroke_width: 1.5

      # Top glowing neon runner line
      - shape: line
        x1: -32
        y1: 26
        x2: 32
        y2: 26
        stroke: "{{neon_accent}}"
        stroke_width: 2

      # Icon circular HUD reticle
      - shape: circle
        x: -38
        y: 0
        r: 13
        fill: "#0f172a"
        stroke: "{{neon_accent}}"
        stroke_width: 1.2
      - shape: icon
        x: -38
        y: 0
        w: 18
        h: 18
        icon: "{{icon}}"

      # System tag and telemetry string
      - shape: text
        x: 12
        y: 6
        text: "{{node_name}}"
        size: 9.5
        color: "#f0fdf4"
      - shape: text
        x: 12
        y: -6
        text: "{{telemetry}}"
        size: 7.5
        color: "{{neon_accent}}"

      # Mini HUD status badge
      - shape: roundedrect
        x: 0
        y: -18
        w: 86
        h: 10
        radius: 2
        fill: "{{badge_bg}}"
      - shape: text
        x: 0
        y: -18
        text: "{{status_code}}"
        size: 6.5
        color: "{{badge_color}}"

      # Cyberpunk Segmented LED Timer Meter
      - shape: progress
        style: segmented
        x: 0
        y: -26
        w: 64
        h: 3
        segments: 6
        gap: 2
        track_color: "#0f172a"
        fill_color: "{{neon_accent}}"
```

### High-Contrast Sci-Fi Palettes

In this theme, each subsystem receives a signature neon accent via `neon_border` and `neon_accent`:
- **Cyan (`#00f5ff`)**: AI Core & Neural Computing
- **Magenta / Hot Pink (`#ff0055`)**: Security Sentinel & Firewall
- **Neon Amber (`#ffb703`)**: Orbital Uplink & Subspace Transceiver

**Try it**: Click **Load this example** below to see the neon telemetry HUD in action with live encrypted packet handshakes.
