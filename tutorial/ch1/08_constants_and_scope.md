# 8. Built-in Constants & State Persistence

When writing reusable node logic in Rhai, you should avoid hardcoding node names or fixed topologies directly into your scripts. Procsim provides built-in scope constants and isolated state management for every node.

---

## 1. Built-in Scope Constants

Every function execution (`on_init`, `on_timer`, `on_msg`) automatically has access to the following constants:

| Constant | Type | Description |
| :--- | :--- | :--- |
| **`node_name`** | `String` | The unique instance name of this node (e.g. `"worker_alpha"`). |
| **`links`** | `Array` | List of downstream peer node names declared in this node's `links: [...]`. |

### Using `node_name` for Contextual Logging & Payloads
Instead of creating separate node types for every worker, you can define a single `worker` node type that dynamically identifies itself:

```rhai
fn on_init() {
  log("[" + node_name + "] initialized and ready");
}

fn on_msg(msg) {
  send(msg.from, #{
    task_id: msg.task_id,
    worker: node_name, // automatically injects "worker_alpha" or "worker_beta"
    display: "ACK from " + node_name
  });
}
```

### Dynamic Routing with `links`
Nodes can inspect `links.len()` and index `links[i]` to distribute traffic without knowing who the peers are ahead of time:

```rhai
fn on_timer() {
  if links.len() == 0 { return; }
  
  let target = links[globals.seq % links.len()];
  globals.seq += 1;
  send(target, #{ task_id: globals.seq });
}
```

---

## 2. State Persistence with `globals`

Local variables declared with `let` inside a function are ephemeral and destroyed when that function finishes.

To persist state across consecutive timer ticks and incoming messages, use the **`globals`** object:

```rhai
fn on_init() {
  globals.processed = 0;
  globals.history = [];
}

fn on_msg(msg) {
  globals.processed += 1;
  globals.history.push(msg.task_id);
  log("[" + node_name + "] total processed: " + globals.processed);
}
```

### Key Rules of `globals`
1. **Isolated per Instance**: Two nodes of the same `node_type` (e.g. `worker_alpha` and `worker_beta`) have completely independent `globals` dictionaries.
2. **Persistent Across Calls**: Values set in `on_init` or modified in `on_msg` are preserved for subsequent `on_timer` and `on_msg` invocations.
3. **Rich Data Types**: `globals` can store numbers, booleans, strings, arrays (`[...]`), and maps (`#{ ... }`).

---

## Try It

1. Load this lesson into the editor and click **Run**.
2. Notice how both `worker_alpha` and `worker_beta` share the identical `worker` node script, yet log their distinct `node_name` and increment their own independent `globals.processed` counters in the Logs panel.
