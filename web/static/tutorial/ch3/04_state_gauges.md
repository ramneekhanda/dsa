# 3.4 Live State Gauges & Meters

By combining Rhai message handlers with `draw()`, nodes can visualize dynamic buffer depths, active request queues, and health metrics directly on canvas.

---

## 1. Computing Visual Geometry from State

When a node receives work or processes items, calculate shape dimensions dynamically:

```rhai
fn update_meter() {
  let q_len = globals.queue.len();
  let status_color = if q_len > 3 { "#ef4444" } else if q_len > 0 { "#f59e0b" } else { "#10b981" };
  let bar_w = (q_len as f32) * 18.0;

  draw([
    #{ shape: "rect", w: 140, h: 60, radius: 8, bg: "#ffffff", border: status_color, border_width: 2 },
    #{ shape: "text", text: "Queue: " + q_len, y: 12, color: "#334155", font_size: 11, bold: true },
    #{ shape: "rect", w: 90, h: 8, radius: 4, y: -8, bg: "#e2e8f0" },
    #{ shape: "rect", w: bar_w.min(90.0), h: 8, radius: 4, x: (bar_w.min(90.0) - 90.0) / 2.0, y: -8, bg: status_color }
  ]);
}
```

---

## 2. Realistic Work Simulation

1. **Producer**: Pushes items downstream on a timer.
2. **Worker**: Enqueues incoming jobs, updates its live load bar, and drains items at a steady rate.
