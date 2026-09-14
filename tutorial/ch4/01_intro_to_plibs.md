# Introduction to Process Libraries (plibs)

Process Libraries (**`plibs`**) are modular, reusable packages of node templates, visual styles, metrics counters, and distributed Rhai handler scripts.

Instead of writing repetitive node drawing routines and network handler functions in every simulation graph, you can import complete domain libraries and themes with a single line.

---

## 1. The `plibs:` Canonical Namespace

`procsim` provides built-in libraries under the canonical **`plibs:`** namespace prefix:

- **`plibs:themes/<name>`**: Complete visual styling packages including canvas background, connection strokes, message bubbles, node cards, and `explain()` narration boxes (`cloud`, `synthwave`, `nordic`, `dracula`, `matrix`, `solarized_light`, `cyberpunk`, `datacenter`, `minimal`).
- **`plibs:aws/<module>`**: Production AWS cloud architecture components (`compute`, `networking`, `database`, `messaging`, and composite `all`).

```yaml
imports:
  - from: "plibs:themes/nordic"
  - from: "plibs:aws/compute"
```

---

## 2. Recursive Import Resolution

`plibs` can import other `plibs`. For example, importing `plibs:aws/all` automatically imports and resolves:

```yaml
imports:
  - from: "plibs:aws/compute"
  - from: "plibs:aws/networking"
  - from: "plibs:aws/database"
  - from: "plibs:aws/messaging"
```

The Rust parser resolves nested dependencies recursively up to 8 levels deep, assembling all node templates, types, icons, and handler functions into a unified simulation graph.

---

## 3. Quick Start Example

Click **▶ Load this example** below to load a lightweight simulation importing the **Nordic** theme package and the **AWS Compute** library running a serverless Lambda handler!
