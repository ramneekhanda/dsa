# 3.4 Minimal Capsule Pills

When designing large distributed network simulations with many interconnected nodes, large cards can take up valuable canvas real estate. **Capsule Pills** provide a clean, horizontal, high-density look that scales gracefully.

### Capsule Geometry & Layout

A capsule pill is composed of a rounded container with a colored left-hand icon disc, a subtle vertical separator, and compact stacked typography:

```yaml
node_templates:
  - id: capsule_pill
    params: [node_name, icon, disc_color, role_tag, tag_color]
    shapes:
      # Outer pill container
      - shape: roundedrect
        x: 0
        y: 0
        w: 114
        h: 32
        radius: 16
        fill: "#ffffff"
        stroke: "#e2e8f0"
        stroke_width: 1.5

      # Left circular icon disc
      - shape: circle
        x: -41
        y: 0
        r: 12
        fill: "{{disc_color}}"
      - shape: icon
        x: -41
        y: 0
        w: 16
        h: 16
        icon: "{{icon}}"

      # Circular progress ring wrapping around the icon disc!
      - shape: progress
        style: ring
        x: -41
        y: 0
        r: 14.5
        thickness: 2
        track_color: "#e2e8f0"
        fill_color: "{{tag_color}}"
        start_angle: 90
        clockwise: true

      # Vertical divider line
      - shape: line
        x1: -24
        y1: 8
        x2: -24
        y2: -8
        stroke: "#e2e8f0"
        stroke_width: 1

      # Title text & role tag
      - shape: text
        x: 16
        y: 4
        text: "{{node_name}}"
        size: 9
        color: "#1e293b"
      - shape: text
        x: 16
        y: -6
        text: "{{role_tag}}"
        size: 7
        color: "{{tag_color}}"
```

### Benefits of Capsule Theming
1. **Compact Footprint**: 114x32 world units versus standard 124x72 cards means 2-3x higher node density without overlap.
2. **Color-Coded Semantic Roles**: The `disc_color` immediately highlights node roles (Blue for Producer, Violet for Broker, Emerald for Consumers).
3. **Clean Connector Terminations**: Connectors meet cleanly at the edges of the smooth rounded pills.

**Try it**: Load the example below to see an event-driven pub/sub mesh rendered with sleek capsule pill nodes.
