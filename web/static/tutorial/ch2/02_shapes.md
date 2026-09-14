# 2.2 Shape Primitives

`draw()`'s shape list understands seven `shape` values. Coordinates are node-local,
`y` up; colors are `#rrggbb` (or `#rgb` / `#rrggbbaa`) hex strings.

| shape | fields | notes |
|---|---|---|
| `rect` | `x, y, w, h, fill, stroke, stroke_width, radius, opacity` | `radius` defaults to `0` (square corners) |
| `roundedrect` | same as `rect` | identical, `radius` just defaults to `8` instead |
| `circle` | `x, y, r, fill, stroke, stroke_width, opacity` | |
| `line` | `x1, y1, x2, y2, stroke, stroke_width` | |
| `polygon` / `polyline` | `points: [[x,y], ...], fill, stroke, stroke_width` | `polygon` closes the shape back to its first point, `polyline` doesn't |
| `text` | `x, y, text, size, color` | |
| `icon` | `x, y, w, h, icon` | `icon` is an id from the graph's `icons:` list - draws that actual icon image, not just a decoration |

Every field except the position/size ones defaults sensibly if omitted - a `circle`
with no `fill`/`stroke` still draws (in a neutral gray), just so a quick shape never
silently disappears for a missing color.

```yaml
node_types:
  - id: gallery
    fn: |
      fn on_timer() {
        draw([
          #{ shape: "rect", x: -70, y: 50, w: 40, h: 24, fill: "#8ecae6" },
          #{ shape: "roundedrect", x: -20, y: 50, w: 40, h: 24, fill: "#8ecae6" },
          #{ shape: "circle", x: 40, y: 50, r: 14, fill: "#ffb703" },
          #{ shape: "line", x1: -90, y1: 10, x2: 90, y2: 10, stroke: "#333333", stroke_width: 2 },
          #{ shape: "polygon", points: [[-20, -20], [20, -20], [0, 20]], closed: true, fill: "#e76f51" },
          #{ shape: "text", x: 0, y: -50, text: "a little gallery", size: 9, color: "#333333" }
        ]);
      }
    attrs:
      ticks: 4
```

Shapes paint in list order - a later shape in the array draws on top of an earlier
one, same as layers in any vector drawing tool.

Notice this draws from `on_timer`, not `on_init`, even though the gallery is entirely
static - that's the gotcha chapter 2.1 flagged: `draw()` in `on_init` doesn't take
effect for a graph-declared node, only `on_timer`/`on_msg` do. Redrawing the exact
same shapes every 2 seconds is harmless (and simpler than reasoning about when a
one-time draw would or wouldn't apply).

**Try it**: load this chapter's example - a single node drawing one of each shape at
once, labeled, so you can see all seven side by side.
