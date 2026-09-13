# Remote Imports & Standard Library Presets

`procsim` allows you to share and reuse themes, node templates, and entire distributed system components through built-in standard library presets and remote URL imports.

---

## 1. Built-in Theme Shorthand

Instead of manually configuring `background`, `connection_color`, `message_theme`, and `node_templates`, you can apply a pre-packaged theme with a single line:

```yaml
theme: cyberpunk
```

Built-in themes include:
- `theme:cyberpunk`: High-contrast dark blueprint with cyan & magenta neon HUD styling.
- `theme:cloud`: Clean, SaaS-style rounded card containers and pill message bubbles.
- `theme:datacenter`: Industrial dark chassis with rack blades and LED status indicators.
- `theme:minimal`: Ultra-clean monochromatic pill capsules.

---

## 2. Standard Library Presets (`stdlib:*`)

Import reusable distributed algorithms directly into your topology:

```yaml
imports:
  - from: "stdlib:load_balancer"
  - from: "stdlib:circuit_breaker"
  - from: "stdlib:cache"
```

You can selectively import and alias node types:

```yaml
imports:
  - from: "stdlib:load_balancer"
    import: ["round_robin_lb"]
    as: "ingress_lb"
```

---

## 3. Remote URL Imports

Import shared component libraries or entire remote microservice topologies hosted on GitHub or HTTP endpoints:

```yaml
imports:
  - from: "https://raw.githubusercontent.com/.../cluster_nodes.yml"
```

The frontend pre-scans and fetches missing remote URLs asynchronously before compiling, seamlessly merging all imported templates and node types with local override precedence!
