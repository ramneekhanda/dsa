# Hierarchical Layouts

ProcSim provides a declarative layout engine that automatically arranges your nodes into clean, deterministic pipelines and architectures.

---

## The `layout` Block

To enable hierarchical layout, define a top-level `layout:` section in your YAML:

```yaml
layout:
  type: hierarchical   # hierarchical, grid, circular, manual
  direction: lr        # lr (left-to-right), tb (top-to-bottom), rl, bt
  rank_sep: 240        # Horizontal distance between ranks (pixels)
  node_sep: 100        # Vertical spacing between nodes in the same rank
```

---

## How Rank Ordering Works

In a hierarchical layout, ProcSim automatically performs topological sorting on the graph links:
- **Root/Source nodes** (nodes that only send messages) are placed at rank 0 on the far left (for `lr`).
- **Intermediate services** (like gateways, workers, filters) are placed in subsequent ranks.
- **Sink/Database nodes** (leaf nodes that receive messages) are aligned at the final rank on the right.

If you omit the `layout:` block, ProcSim automatically defaults to a clean `lr` hierarchical layout!

---

## Locking & Interactivity

By default, nodes in a layout are locked in place (`LayoutLocked`) so that dragging or accidental clicks do not disturb the diagram structure. You can still pan the canvas and zoom in/out with your mouse wheel.

Click **▶ Load this example** above and run the simulation to see the message flow smoothly through the 4-stage pipeline!
