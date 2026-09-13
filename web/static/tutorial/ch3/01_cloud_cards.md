# 3.1 Cloud Architecture Theme

Welcome to Chapter 3! Now that you know how `node_templates` work with `{{param}}` placeholders, let's explore how templates can transform your simulations from simple icon-and-label graphs into professional, presentation-ready architecture diagrams.

In modern cloud architecture diagrams (AWS, GCP, Azure, or Kubernetes topologies), services are typically rendered as **rich cards** with distinct visual hierarchies:
- A clean card body with subtle borders and drop-surface contrast
- A branded **color stripe** indicating the service tier (e.g. edge routing, business logic, storage)
- An **icon emblem container** on the left
- A **title** and **category subtitle**
- A **status or metadata pill** along the bottom

```yaml
node_templates:
  - id: cloud_card
    params: [node_name, icon, accent_color, service_type, status_bg, status_color, status_text]
    shapes:
      # Main card body
      - shape: roundedrect
        x: 0
        y: 0
        w: 124
        h: 72
        radius: 8
        fill: "#ffffff"
        stroke: "#cbd5e1"
        stroke_width: 1.5

      # Top accent banner
      - shape: rect
        x: 0
        y: 32
        w: 124
        h: 8
        fill: "{{accent_color}}"

      # Icon circular container & icon image
      - shape: circle
        x: -38
        y: 6
        r: 16
        fill: "#f8fafc"
        stroke: "#e2e8f0"
        stroke_width: 1
      - shape: icon
        x: -38
        y: 6
        w: 22
        h: 22
        icon: "{{icon}}"

      # Node Name & Service Subtitle
      - shape: text
        x: 14
        y: 12
        text: "{{node_name}}"
        size: 10
        color: "#0f172a"
      - shape: text
        x: 14
        y: -2
        text: "{{service_type}}"
        size: 8
        color: "#64748b"

      # Bottom status pill
      - shape: roundedrect
        x: 0
        y: -20
        w: 108
        h: 14
        radius: 4
        fill: "{{status_bg}}"
      - shape: text
        x: 0
        y: -20
        text: "{{status_text}}"
        size: 7.5
        color: "{{status_color}}"

      # Integrated bottom timer progress bar
      - shape: progress
        style: bar
        x: 0
        y: -34
        w: 124
        h: 4
        track_color: "#f1f5f9"
        fill_color: "{{accent_color}}"
```

### Why this is powerful

Notice how the `cloud_card` template encapsulates 7 distinct shape primitives into a reusable component. Each node type (`api_gateway`, `microservice`, `cache`, `database`) simply references `template_ref: cloud_card` and provides its own `template_params`:

- `accent_color`: `#f59e0b` (Amber for Gateway), `#3b82f6` (Blue for Services), `#8b5cf6` (Purple for Cache), `#10b981` (Green for Database)
- `service_type`: Subtitle clarifying role (e.g. `"Edge / Ingress"`, `"Managed DB"`)
- `status_text`: Operational badge (e.g. `"HEALTHY • 240 RPS"`, `"REPLICA SYNCED"`)

**Try it**: Click **Load this example** below. Notice how each node renders a polished, production-grade cloud card while messages flow seamlessly across the architecture!
