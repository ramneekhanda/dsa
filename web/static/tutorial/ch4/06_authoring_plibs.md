# Authoring Custom `plibs`

You can create and distribute your own reusable `plibs` packages for internal architectures, custom cloud providers, or algorithmic benchmarks.

---

## 1. Anatomy of a Library File

A `plib` YAML file contains reusable sections:

```yaml
# 1. Custom SVG Icon symbols
icons:
  - id: custom_shield
    svg: '<path d="M12 2L3 7v6c0 5.5 3.8 10.7 9 12 5.2-1.3 9-6.5 9-12V7l-9-5z"/>'

# 2. Parametric Node Templates
node_templates:
  - id: security_gateway_card
    width: 140
    height: 60
    draw: |
      draw_rect(0, 0, width, height, #{ fill: "#111827", stroke: "#10B981", stroke_width: 2, rx: 6 });
      draw_text(10, 20, attrs.title, #{ fill: "#FFFFFF", font_size: 11, font_weight: "bold" });
      draw_text(10, 40, "STATUS: " + attrs.status, #{ fill: "#10B981", font_size: 9 });

# 3. Rhai Behavior Scripts
fns:
  - &sec_gw_fn |
    fn on_message(from_node, msg) {
      log("Inspecting message from " + from_node);
      send_all(msg);
    }

# 4. Default Node Types
graph_defn:
  node_types:
    - id: security_gateway
      template: security_gateway_card
      fn: *sec_gw_fn
      attrs:
        title: "WAF GATEWAY"
        status: "FILTERING"
```

---

## 2. Packaging and Sharing

1. **Local Project Folder**: Place your files in a local `plibs/` folder in your project.
2. **Git Repository / HTTP Hosting**: Host them on GitHub or S3 and import directly via URL:
   ```yaml
   imports:
     - from: "https://raw.githubusercontent.com/my-org/my-procsim-libs/main/security.yml"
   ```

---

## 3. Interactive Example

Click **▶ Load this example** below to test a self-contained custom library definition!
