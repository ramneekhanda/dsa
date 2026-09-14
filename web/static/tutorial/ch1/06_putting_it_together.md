# 6. Putting It Together

One last example, combining everything so far into a small, familiar shape: a client
that retries on failure, and a server that's sometimes (configurably) unreliable.

```yaml
node_types:
  - id: client
    fn: |
      fn on_init() {
        globals.req_id = 0;
        globals.retries = 0;
      }
      fn on_timer() {
        globals.req_id += 1;
        send("server", #{ req_id: globals.req_id, display: "req #" + globals.req_id, icon: "request_icon" });
      }
      fn on_msg(msg) {
        if msg.ok {
          log("client got a reply for #" + msg.req_id);
        } else {
          globals.retries += 1;
          log("client got an error for #" + msg.req_id + " - retrying (" + globals.retries + " so far)");
          send("server", #{ req_id: msg.req_id, display: "retry #" + msg.req_id, icon: "request_icon" });
        }
      }
    attrs:
      ticks: { min: 4, max: 7, jitter: true }

  - id: server
    fn: |
      fn on_msg(msg) {
        let ok = true;
        if flaky {
          ok = !random_chance(unreliable_pct);
        }
        send("client", #{ req_id: msg.req_id, ok: ok, display: "reply #" + msg.req_id, icon: "response_icon" });
      }
    params:
      - name: flaky
        type: Bool
        default: true
      - name: unreliable_pct
        type: Integer
        default: 25
        min: 0
        max: 100
```

Nothing here is new - `links`/`send`/`on_msg` from chapter 3, `params` from chapter 4,
`globals`/`random_chance` from chapter 5 - only combined into something that actually
resembles a real distributed system's failure-handling behavior.

**Try it**: load this chapter's example and run it. Then, while it's running,
double-click the `server` node and push `unreliable_pct` up toward 80-90 - watch the
client's retry count in the Logs panel climb much faster.

## Where to go from here

This covers the fundamentals: graph structure, node types, messaging, params, icons,
state, and logging - enough to read and build most simulations. That's this whole
chapter - **Chapter 1: Fundamentals**.

**Chapter 2: Custom Visuals**, up next in the chapter list, picks up right where this
leaves off: giving a node its own custom on-canvas look with `draw()`, and reusable
node templates so many node types can share one consistent style.
