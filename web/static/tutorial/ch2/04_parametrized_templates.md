# 2.4 Parametrized Templates

A template's string fields (`text`, `fill`, `stroke`, `color`, `icon`) can embed
`{{param}}` placeholders, filled in separately for each node type/instance that uses
the template. Two params are always available for free, no wiring needed:

- **`{{node_name}}`** - that specific node *instance*'s own name
- **`{{icon}}`** - that node *type*'s own `attrs.icon`

```yaml
node_templates:
  - id: badge
    params: [node_name, icon]
    shapes:
      - shape: icon
        x: 0
        y: 0
        icon: "{{icon}}"
      - shape: roundedrect
        x: 0
        y: -34
        w: 92
        h: 20
        fill: "#1b3a5c"
      - shape: text
        x: 0
        y: -34
        text: "{{node_name}}"
        size: 10
        color: "#ffffff"

node_types:
  - id: client
    attrs:
      icon: client_icon
      template_ref: badge
  - id: server
    attrs:
      icon: server_icon
      template_ref: badge
```

Now `client` and `server` share one template definition, yet each renders its own
icon, and each *instance* (`client_a`, `client_b`, ...) shows its own name - one
definition, correct everywhere it's used.

Need to override what a placeholder shows? `attrs.template_params` is an explicit
map, checked after the two automatic bindings, so it can replace either one (or add
params of your own the template refers to):

```yaml
node_types:
  - id: server
    attrs:
      template_ref: badge
      template_params:
        node_name: "Primary Server"   # shown on the badge instead of the real name "server"
```

Everything else about that node - its real name, what `links`/`send()` address it
by - is unaffected; only the rendered label text changes.

**Try it**: load this chapter's example - `client_a` and `client_b` are the same node
type, sharing one template, yet each correctly shows its own name; `server` shows the
friendlier "Primary Server" label via `template_params`, overriding its real name.

This is the last chapter 2 lesson - between `draw()`, the shape primitives, and
templates (plain, shared, and parametrized), you now have the full toolkit for giving
a graph a custom, consistent visual identity beyond the default icon+label look.
