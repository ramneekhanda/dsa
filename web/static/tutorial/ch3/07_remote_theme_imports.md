# Remote Themes & Package Libraries

`procsim` makes it easy to compose distributed system simulations by combining remote themes, reusable component libraries, and custom algorithmic logic.

---

## 1. Importing Theme Packages

Instead of manually defining node shapes, colors, connector styles, message bubbles, and callout themes in every graph, you can import complete packages from **`plibs:`**, relative library paths, or any remote HTTP/HTTPS URL:

```yaml
imports:
  - from: "plibs:synthwave"
  - from: "stdlib:load_balancer"
```

You can also import directly from relative library files or remote repositories:

```yaml
imports:
  - from: "plibs/themes/synthwave.yml"
  - from: "https://raw.githubusercontent.com/your-org/procsim-libs/main/themes/dracula.yml"
```

---

## 2. What a Theme Package Includes

When a theme package is imported, the engine seamlessly merges:

1. **`graph_attrs`**:
   - Canvas `background`, animated `connection_color`, and primary `text_color`.
   - **`message_theme`**: Custom bubble shape (`chamfered`, `rounded`, `box`, `pill`), background fill, stroke color/width, and font typography for messages in flight.
   - **`explain_theme`**: Custom shape, background, border, accent button, glow shadow, and backdrop dimming for `explain()` narration callouts.
2. **`node_templates`**:
   - Parameterized vector shapes (`synth_card`, `nord_card`, `dracula_node`, `matrix_blade`, etc.) with headers, status tags, segmented activity meters, and LED indicators.
3. **`icons`**:
   - Reusable SVG icon symbols referenced across node templates.
4. **`node_types`**:
   - Pre-configured node type definitions that can be instantiated directly or extended with custom Rhai scripts.

---

## 3. Composing Algorithms with Themes

You can combine modular themes with standard library algorithms (`stdlib:load_balancer`, `stdlib:circuit_breaker`, `stdlib:cache`) and override specific fields locally:

```yaml
imports:
  - from: "plibs:synthwave"
  - from: "stdlib:load_balancer"
    import: ["round_robin_lb"]

graph_defn:
  graph_attrs:
    title: "Synthwave Cluster with Round-Robin LB"
  graph:
    - name: client
      node_type: client
      links: [lb]
    - name: lb
      node_type: round_robin_lb
      links: [srv1, srv2]
    - name: srv1
      node_type: worker
      links: []
    - name: srv2
      node_type: worker
      links: []
```

Click **Load this example** below to see the **Synthwave** theme package and standard library load balancer running in action with interactive `explain()` callouts!
