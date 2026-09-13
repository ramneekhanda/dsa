# 1. Introduction

**procsim** is a visual simulator for distributed systems. You describe a graph of
**nodes** and the **links** between them in a small YAML document; each node runs a
little script (in a language called [Rhai](https://rhai.rs), which reads a lot like
JavaScript) that reacts to a timer and to messages arriving from other nodes. Run the
graph and watch messages animate along the connectors while each node's own log
output streams into the **Logs** panel.

It's a good way to build intuition for how distributed patterns actually behave over
time - retries, timeouts, leader election, load balancing - by watching a small,
readable simulation of them rather than just reading about them.

## The workspace

- **Editor** (this side, when a chapter isn't open) - a YAML editor with live
  autocomplete and validation, powered by a JSON Schema generated straight from
  procsim's own Rust data model. It will complain if you typo a field name.
- **View** - the canvas. Nodes render as icons with a name label; connectors are the
  curved lines between linked nodes; small icons travel along a connector while a
  message is in flight.
- **Logs** - every `log(...)` call any node's script makes, newest first, with a
  timestamp.
- **Learn** - this tutorial.
- **Run** (top toolbar) - (re-)compiles the current YAML and loads it into the
  simulation. Click it any time you change the editor's contents.

Two more things worth knowing before you start:

- **Double-click a node** to open its own properties popup - if its node type has
  configurable `params`, you can edit them live, right there, no reload needed.
- **Double-click empty canvas** to open **Graph Properties** - simulation speed, and
  the graph's background/text/connector colors.
- **Scroll to zoom, drag to pan** the canvas (right-click-drag to pan too); drag a
  node itself to reposition it.

## How this tutorial works

Each following chapter is a short, focused concept with a runnable YAML snippet
attached. Click **Load this example** at the top of a chapter to drop its YAML
straight into the Editor, then click **Run** to try it - read the log output, watch
the canvas, and feel free to tweak the YAML and re-run. That loop - read, run,
tweak - is the fastest way through this tutorial.

Next up: **2. Your First Graph**, in the chapter list on the left.
