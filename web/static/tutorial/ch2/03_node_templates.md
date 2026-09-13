# 2.3 Reusable Node Templates

Calling `draw()` from `on_init` every time you want a static look is a lot of
boilerplate, and doesn't help two node types that should share the same style. A
**template** declares that same shape list directly in YAML instead - no script
required at all:

```yaml
node_types:
  - id: client
    attrs:
      template:
        - shape: circle
          x: 0
          y: 0
          r: 20
          fill: "#3fae49"
```

`attrs.template` is a list of the exact same shapes chapter 2.2 covered, just spelled
in YAML instead of a Rhai array - and it fully replaces that node type's default
icon+label, the same as a `draw()` call would.

For a look shared by *several* node types, declare it once under the graph's
top-level `node_templates:` list, and point `attrs.template_ref` at its `id`:

```yaml
graph_defn:
  node_templates:
    - id: badge
      shapes:
        - shape: icon
          x: 0
          y: 0
          icon: gizmo_icon
        - shape: roundedrect
          x: 0
          y: -34
          w: 70
          h: 20
          fill: "#1b3a5c"
        - shape: text
          x: 0
          y: -34
          text: "node"
          size: 10
          color: "#ffffff"
  node_types:
    - id: client
      attrs:
        template_ref: badge
    - id: server
      attrs:
        template_ref: badge
```

**Try it**: load this chapter's example - `client` and `server` are different node
types, but both render the exact same badge, because both reference the same
`node_templates` entry.

Notice both nodes look *completely* identical here, right down to the label text
("node") - a fixed template has no way to say "but show this node's own name and
icon". Chapter 2.4 is exactly that missing piece.
