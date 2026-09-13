# 3.5 Layered & Composed Themes

One of the most powerful architectural features of `procsim`'s theming system is **Template Composition**.

When a node type specifies both `template_ref` and its own inline `attrs.template` shapes:
1. The referenced `template_ref` shapes render **first** (the foundation layer).
2. The inline `attrs.template` shapes render **on top** (the accent/custom layer).

This allows you to establish a single, standardized base template for your entire network while letting specialized node roles (like a Raft Cluster Leader or an active failover node) layer distinct badges or halo accents on top without duplicating the base template!

```yaml
node_templates:
  # Standard base node template used across the entire cluster
  - id: cluster_card
    params: [node_name, icon, role_label, border_color]
    shapes:
      - shape: roundedrect
        x: 0
        y: 0
        w: 110
        h: 60
        radius: 8
        fill: "#0f172a"
        stroke: "{{border_color}}"
        stroke_width: 1.5
      - shape: icon
        x: -30
        y: 0
        w: 24
        h: 24
        icon: "{{icon}}"
      - shape: text
        x: 14
        y: 8
        text: "{{node_name}}"
        size: 9.5
        color: "#f8fafc"
      - shape: text
        x: 14
        y: -8
        text: "{{role_label}}"
        size: 7.5
        color: "#94a3b8"

node_types:
  # Followers use the base template directly
  - id: follower
    attrs:
      icon: node_icon
      template_ref: cluster_card
      template_params:
        border_color: "#334155"
        role_label: "FOLLOWER"

  # The Leader inherits the base cluster_card, but layers a golden crown on top!
  - id: leader
    attrs:
      icon: node_icon
      template_ref: cluster_card
      template_params:
        border_color: "#eab308"
        role_label: "LEADER (TERM 4)"
      # Shapes drawn ON TOP of the base card:
      template:
        # Outer golden halo
        - shape: roundedrect
          x: 0
          y: 0
          w: 116
          h: 66
          radius: 10
          stroke: "#eab308"
          stroke_width: 1
          opacity: 0.5
        # Top-right Golden Leader Crown Badge
        - shape: circle
          x: 48
          y: 24
          r: 9
          fill: "#eab308"
        - shape: polygon
          points:
            - [44, 21]
            - [46, 26]
            - [48, 23]
            - [50, 26]
            - [52, 21]
          closed: true
          fill: "#0f172a"
        # Leader Heartbeat Timer Ring around its icon
        - shape: progress
          style: ring
          x: -32
          y: 0
          r: 18
          thickness: 2
          track_color: "#1e293b"
          fill_color: "#eab308"
          start_angle: 90
          clockwise: true
```

### Why Layering Matters
- **DRY (Don't Repeat Yourself)**: If you adjust the base card width or font sizes in `cluster_card`, every node type updates automatically.
- **Role Badging**: Special roles (Leaders, Coordinators, Degraded Nodes, Watchers) can add visual markers (crowns, warning triangles, halo glows) without needing separate whole-card templates.

**Try it**: Click **Load this example** below to see a Raft Consensus cluster where the elected Leader node layers a golden status badge and halo glow over the shared cluster template while sending periodic heartbeats to its followers.
