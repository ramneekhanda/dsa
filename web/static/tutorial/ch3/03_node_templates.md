# 3.3 Reusable Node Templates

Instead of manually repeating shape arrays in every script, you can declare declarative templates under `node_templates` in `graph_defn`.

---

## 1. Defining `node_templates`

```yaml
graph_defn:
  node_templates:
    - id: server_card
      template:
        - shape: rect
          w: 130
          h: 55
          radius: 6
          bg: "#ffffff"
          border: "#0284c7"
          border_width: 2
        - shape: text
          text: "SERVICE"
          y: 12
          color: "#0369a1"
          font_size: 10
```

---

## 2. Referencing Templates in Node Types

Attach the template to any node type via `attrs.template_ref`:

```yaml
  node_types:
    - id: api_server
      attrs:
        template_ref: server_card
        ticks: 4
```

Any node using this type will automatically render with the template's shapes. Handlers can still call `draw()` at runtime to add dynamic badges or overlay progress meters on top.
