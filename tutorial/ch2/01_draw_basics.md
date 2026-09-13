# 2.1 draw() Basics

Every node so far has looked the same: a default icon, a name label underneath. A
handler can replace that entirely with its own custom, live-updating drawing, using
the `draw()` host function:

```
fn draw(shapes: array);
```

`shapes` is a list of shape maps, each with a `shape` key selecting what it is - the
full list is chapter 2.2's subject. `draw()` paints in the node's own local
coordinate space: `(0, 0)` is the node's center, `x` right, `y` up, in the same world
units as everything else on the canvas.

```yaml
node_types:
  - id: light
    fn: |
      // Rhai functions can't see the caller's script-level vars (globals
      // included) - viz() has to take everything it needs as a parameter.
      fn viz(is_on, ticks) {
        let color = "#d1453b";
        if is_on { color = "#3fae49"; }
        draw([
          #{ shape: "circle", x: 0, y: 0, r: 20, fill: color, stroke: "#000000", stroke_width: 1.5 },
          #{ shape: "text", x: 0, y: -34, text: "tick " + ticks, size: 11, color: "#333333" }
        ]);
      }
      fn on_init() {
        globals.ticks = 0;
        globals.on = false;
        viz(globals.on, globals.ticks);
      }
      fn on_timer() {
        globals.ticks += 1;
        globals.on = !globals.on;
        viz(globals.on, globals.ticks);
      }
    attrs:
      ticks: 2
```

Three things worth internalizing early, because they explain a lot of surprising
behavior later:

- **`draw()` replaces the whole overlay**, every time it's called - it's not additive.
  Calling it again with fewer shapes means fewer shapes get drawn, not "the old ones
  plus the new ones".
- A node with no template (chapter 2.3) and no `draw()` call at all just keeps its
  ordinary default icon+label - `draw()` only takes over once a handler actually calls
  it.
- **A helper function like `viz()` can't read `globals` (or any other script-level
  variable) directly, even though it's defined right there in the same script** -
  Rhai functions only ever see their own parameters and locals. Pass whatever the
  helper needs (`is_on`, `ticks` above) as arguments instead. Forgetting this shows up
  as a silent "nothing drew" with an error in the browser console
  (`ErrorVariableNotFound("globals", ...)`), not a YAML/schema error - easy to miss the
  first time.

**A separate, unrelated gotcha while we're here**: a `draw()` call in `on_init`
doesn't actually show anything for a node declared in the graph's YAML (only for a
node a script spawns at runtime, a more advanced topic than this tutorial covers).
`on_init` still runs and still sets up `globals` normally - it's specifically the
drawing that's silently skipped there. `on_timer`/`on_msg` don't have this limitation,
so the light above still works correctly - it just won't show anything until its
first tick, 2 seconds after loading, rather than the instant you hit Run.

**Try it**: load this chapter's example, run it, and watch the light flip between red
and green every 2 seconds, with a running tick count above it - both entirely drawn by
the script, no default icon in sight. Give it those first 2 seconds before you go
looking for a bug, per the note above.
