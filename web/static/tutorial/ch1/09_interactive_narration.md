# 9. Interactive Narration with `explain()`

When building educational simulations, architectural diagrams, or debugging complex state transitions, you often want the simulation to pause at key moments and guide the viewer with contextual explanations.

Procsim provides the built-in host function **`explain(key, text)`** for in-canvas interactive narration bubbles.

---

## 1. How `explain(key, text)` Works

When any node calls `explain(key, text)`:
1. **Simulation Pauses**: Virtual time freezes all timer ticks, in-flight message bubbles, and node state updates.
2. **Radial Blur**: The background canvas dims and blurs, highlighting the active node.
3. **Narration Bubble**: A high-contrast callout bubble pops up directly above the node displaying `text`.
4. **Continue Button**: Clicking **Continue** on the bubble dismisses it, showing the next queued explanation or unpausing the simulation when the queue is cleared.
5. **Deduplication (`key`)**: Each `key` is shown at most **once per run**, preventing repeated triggers on consecutive ticks.

```rhai
fn on_timer() {
  globals.count += 1;
  if globals.count == 1 {
    explain("intro", "Welcome! This node coordinates downstream tasks.");
  }
}
```

---

## 2. Triggering Explanations on Events

You can call `explain()` conditionally on specific timer ticks or upon receiving incoming messages:

```rhai
fn on_msg(msg) {
  // Triggers once the first time a message arrives
  explain("first_intercept", "Gateway intercepted " + msg.display + " and will forward it to worker.");
  
  send(links[0], msg);
}
```

---

## Try It

1. Load this lesson into the editor and click **Run**.
2. On the first timer tick, `client` will pause the simulation and display an introductory narration bubble.
3. Click **Continue** on the bubble to proceed.
4. When `gateway` intercepts the request, the simulation automatically pauses again to explain the routing logic before passing the task to `worker_1`.
