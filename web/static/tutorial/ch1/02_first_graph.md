# 2. Your First Graph

Every procsim document has the same top-level shape:

```yaml
graph_defn:
  graph:        # the actual nodes and links (the instances)
    # ...
  graph_attrs:  # graph-wide settings: title, colors
    # ...
  node_types:   # reusable node type definitions (the classes)
    # ...
```

`graph` and `node_types` are deliberately separate: a **node type** is a reusable
definition (its script, its timer interval, its configurable params) and `graph` is
the list of actual **node instances** in your simulation, each pointing at one
`node_type` by id. Many nodes can share one type.

Here's the smallest graph that does something - one node, ticking every 4 seconds:

```yaml
graph_defn:
  graph:
    - name: pinger
      node_type: pinger
      links: []
  node_types:
    - id: pinger
      fn: |
        fn on_init() {
          log("pinger is alive");
        }
        fn on_timer() {
          log("tick!");
        }
      attrs:
        ticks: 4
```

A node type's `fn` is a Rhai script defining up to three handlers, all optional:

- **`on_init()`** - runs once, when the node is created (graph load, or a runtime
  `spawn_node()` later on).
- **`on_timer()`** - runs every `attrs.ticks` seconds.
- **`on_msg(msg)`** - runs when a message arrives (chapter 3).

`log(some_string)` is how a script talks to you - it shows up in the **Logs** panel
with a timestamp, tagged with the node's name.

**Try it**: load this chapter's example (button above), hit **Run**, then open the
**Logs** tab and watch a `tick!` line appear every 4 seconds.

`attrs.ticks` doesn't have to be a fixed number - `ticks: { min: 2, max: 5 }` picks one
random interval when the node is created; adding `jitter: true` re-picks a fresh
random interval every single time the timer fires, instead of a perfectly regular
metronome. More on this in chapter 4.
