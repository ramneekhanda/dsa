# 4. Configurable Node Types

A node type can declare typed **params** - knobs an author (or a reader experimenting
with your graph) can adjust without touching the YAML at all, live, while the
simulation runs.

```yaml
node_types:
  - id: greeter
    fn: |
      fn on_timer() {
        if shout {
          log("HELLO #" + enthusiasm);
        } else {
          log("hello #" + enthusiasm);
        }
      }
    params:
      - name: enthusiasm
        type: Integer
        default: 1
        min: 1
        max: 5
      - name: shout
        type: Bool
        default: false
    attrs:
      ticks: { min: 2, max: 4, jitter: true }
      icon: greeter_icon
```

Each param becomes a bare variable inside that node's handlers - `enthusiasm` and
`shout` above, no `params.` prefix. Five types are available: `Bool`, `Integer`,
`Float`, `String`, and `Option` (a fixed dropdown of string values) - `Integer`/
`Float` take `min`/`max`, `Option` takes a `values` list.

**Try it**: load this chapter's example, run it, then **double-click the node** on the
canvas - a popup opens with `enthusiasm` and `shout` as live controls. Change them and
watch the very next tick's log line change too.

## Icons

`attrs.icon` sets a node type's on-canvas icon, referencing an id from the graph's
top-level `icons:` list:

```yaml
graph_defn:
  icons:
    - id: greeter_icon
      url: https://example.com/some-icon.png
  node_types:
    - id: greeter
      attrs:
        icon: greeter_icon
```

Any node type that doesn't set `attrs.icon` gets a plain default icon. The same
`icons:` registry is what a message's own `icon` field (chapter 3) references too -
one shared list for both.
