# 5. State and Logging

A handler's local variables don't survive between calls - each `on_timer`/`on_msg`
starts fresh. To remember something across ticks or messages (a counter, a running
total, a flag), use **`globals`**: a persistent map, unique to each node instance,
that's there waiting for you on every call.

```yaml
node_types:
  - id: counter
    fn: |
      fn on_init() {
        globals.count = 0;
        globals.failures = 0;
      }
      fn on_timer() {
        globals.count += 1;
        if random_chance(30) {
          globals.failures += 1;
          log("tick " + globals.count + " - simulated failure (" + globals.failures + " so far)");
        } else {
          log("tick " + globals.count + " - ok");
        }
      }
    attrs:
      ticks: 2
```

Seed `globals` in `on_init` (it starts out empty otherwise) and read/write it anywhere
else. Two nodes of the same type never share `globals` - each instance gets its own.

`random_chance(percent)` is the other new thing here: it returns `true` roughly
`percent`% of the time. It's the standard way to simulate flaky, unreliable behavior
(a request that sometimes fails, a node that's sometimes down) without a manually
toggled param - chapter 6 puts it to real use.

**Try it**: load this chapter's example, run it, and watch the Logs panel - the
failure count only ever goes up, never resets, because it lives in `globals` rather
than a local variable that would reset every tick.

The **Logs** panel itself is worth a closer look while you're there: every row is one
`log()` call, newest at the top, with a timestamp - it's usually the fastest way to
see what a graph is actually doing tick by tick, especially once a graph has more than
one or two nodes running at once.
