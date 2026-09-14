# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

`procsim` (Cargo package name `dsa`) is a browser-based visual simulator for distributed
systems / process graphs. A Rust/Bevy (ECS) app compiles to WebAssembly and renders into a
`<canvas>` embedded in a SvelteKit app. Users write a YAML document (edited live in a Monaco
editor with schema-driven autocomplete) that defines a graph of nodes; each node type carries
a Rhai script with `on_init` / `on_timer` / `on_msg` handlers. The engine ticks node timers,
runs the Rhai handlers, and animates messages travelling along the connectors between nodes.

Two halves that must both be understood together:
- `src/` — the Rust/Bevy simulation engine, compiled to `wasm32-unknown-unknown` and exposed to
  JS via `wasm-bindgen`.
- `web/` — a SvelteKit + Monaco + dockview frontend that hosts the compiled wasm module, the
  YAML code editor, and the log/inspector panels.

## Build & run commands

All top-level orchestration goes through `cargo-make` (`Makefile.toml`), which drives both the
Rust→wasm build and the npm/SvelteKit build. Run these from the repo root.

```bash
# one-time
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo install cargo-make

# fast local iteration: dev-profile wasm build + `vite dev` with hot reload
cargo make run-dev

# production-style: release-profile wasm build (cargo-make "production" profile),
# then builds the static site and serves it via `cargo server`
cargo make -p production run

# just build both halves without serving (used in CI)
cargo make -p production build-all

# desktop app (Tauri shell around the same SvelteKit + wasm build - see
# "Desktop shell" below): dev mode with hot reload, or a release .app/.dmg bundle
cargo make tauri-dev
cargo make -p production tauri-build

# rust-only checks
cargo build
cargo test
cargo check --target wasm32-unknown-unknown
cargo bench     # microbenchmarks, e.g. benches/rhai_state_roundtrip.rs - native-only,
                 # doesn't touch wasm/bevy, HTML report at target/criterion/report/index.html

# frontend-only checks (from web/)
npm run check        # svelte-check + svelte-kit sync
npm run dev           # vite dev server alone (needs wasm already built once, see below)
npm run build
```

Default (no `-p production`) cargo-make profile is "development": faster/unoptimized wasm
build (`[env.development]` in `Makefile.toml`). Passing `-p production` switches to the
`[env]` defaults: `lto`, `opt-level = 'z'`, single codegen unit, stripped debuginfo — this is
what CI (`.github/workflows/rust.yml`) uses to build and deploy to GitHub Pages.

Key generated artifacts (all under the gitignored `package/`):
- `package/wasm/dsa.js` / `dsa_bg.wasm` — wasm-bindgen output.
- `package/release/` — the final static site (SvelteKit adapter-static output +
  `assets/fonts`), what gets deployed.

`web/src/routes/dsa.js` is a **checked-in symlink** to `../../../package/wasm/dsa.js`. This
means the frontend cannot resolve its wasm import until the Rust side has been built at least
once (`cargo make build-wasm-module` or any of the `run*`/`build-all` tasks) — a bare
`npm run dev` in `web/` on a clean checkout will fail to load the module.

There is no dedicated Rust test suite beyond `cargo test`/`Makefile.toml`'s `test` task
(no `#[test]` functions currently exist in `src/`); correctness is mostly exercised by loading
example graphs through the running app. Example/sample graph YAML lives in `examples/config/`
(loaded from the frontend's "examples" menu, fetched at runtime from `examples/`).
`benches/` (native-only, `criterion`, see `Cargo.toml`'s `[[bench]]` entry) exists
separately for performance, not correctness — currently just
`rhai_state_roundtrip.rs` (see the state round-trip note in the tick-loop section below).

## Architecture

### Graph definition & YAML schema (`src/parser/graphv2.rs`)

This is the single source of truth for the on-disk/YAML data model and is intentionally the
most important file to read first:
- `File { graph_defn: GraphDefinition }` is the YAML root.
- `GraphDefinition` has `node_types` (reusable type definitions, each with an optional Rhai
  `fn` script, `attrs` (a tick interval and an on-canvas `icon` id), and typed `params` —
  Bool/Float/Integer/String/Option), `graph` (the actual node instances as
  `NodeConnection { name, node_type, links }`), `graph_attrs` (colors/title), and `icons`
  (a named `IconDef { id, url }` registry — `attrs.icon` and a `send()` payload's own
  `icon` key both reference an entry here by `id`; `parse_graph2` validates `attrs.icon`
  against this list the same way it validates `graph[].node_type` against `node_types`).
  `attrs.ticks` is a `Ticks` enum (untagged, so plain `ticks: 5` still works): either a
  fixed seconds value, or `{min, max}` resolved to one random value at load time
  (`resolve_ticks_secs`), or `{min, max, jitter: true}` which re-resolves to a fresh
  random value every time the timer fires (handled in `rhai_engine.rs`'s tick loop via
  `Timer::set_duration`, safe to call because `Timer::tick()` only reads whatever
  duration is currently set, verified against `bevy_time`'s source).
- `parse_graph2()` deserializes the YAML, **compiles each node type's Rhai script into an
  `AST`** (`compile_ast`), then instantiates a `Node` per graph entry (looking up its
  `NodeType`, creating a per-node Rhai `Scope` and a `Timer` from `attrs.ticks`), and runs
  `on_init` once per node via `init_scope` to seed each node's persistent `state`
  (`Dynamic`, stored under the `state` scope variable).
- `schemars` derives (`JsonSchema` on nearly everything) are what generate the JSON Schema
  served to Monaco (`get_code_schema` in `src/systems/ingest_code.rs`) for live YAML
  autocomplete/validation in the editor — changing these structs changes the editor's schema.
- `NodeType.id` (`#[serde(alias = "name")]`) accepts either `id:` or `name:` in a
  `node_types` entry — added so hand-written and import-merged YAML (see imports below,
  which key/dedup `node_types` by `id`) can use whichever reads better without a schema
  fork; `parse_graph2`/`merge_graph_definitions` only ever see the normalized `id`.

### Imports & module presets (`src/parser/imports.rs`, `plibs/`)

A graph can pull in reusable `node_types`/`icons`/`node_templates`/`layout`/`groups` from
another YAML document instead of redefining them inline — `graph_defn.imports` (or the
top-level shorthand `imports:`) is a list of `ImportDef { from, import: Option<Vec<String>>,
as: Option<String> }`. `from` is either a built-in preset name/shorthand (resolved by
`get_builtin_preset`, matched case-insensitively against a long alias list — e.g.
`theme:cyberpunk`/`cyberpunk`/`plibs:themes/cyberpunk` all hit the same
`include_str!("../../plibs/themes/cyberpunk.yml")`) or a remote `http(s)://` URL. There's
also a bare `theme: <name>` shorthand on `File`/`GraphDefinition` that's sugar for an
`ImportDef` with no `import`/`as` filter, resolved the same way.
- **Preset families under `plibs/`**: `themes/` (cyberpunk, cloud, datacenter, minimal,
  synthwave, nordic, dracula, matrix, solarized_light — each a `node_templates` +
  `graph_attrs.message_theme`/`explain_theme` bundle), `aws/` (compute/networking/database/
  messaging/all — icon + node_type libraries for cloud-architecture diagrams), and a few
  fully inline Rust-string stdlib presets in `imports.rs` itself (`stdlib:load_balancer`
  → `round_robin_lb`/`weighted_lb`, `stdlib:circuit_breaker`, `stdlib:cache` → `lru_cache`)
  that don't warrant their own `plibs/` file.
- **Remote imports are resolved outside Rust**: `resolve_file_imports` takes an
  `external_sources: &HashMap<String, String>` (URL → already-fetched YAML text) rather
  than fetching itself — `scan_import_urls` does a fast heuristic line-scan (falls back to
  a full `File` parse) so the frontend/wasm caller can `fetch()` every referenced URL first
  and hand the results back in; an unresolved `http(s)://` import is a hard parse error.
- `resolve_file_recursive` recurses per import (depth-capped at 8, same
  cyclic/runaway-growth concern as `MAX_NODES`) and folds each resolved import into the
  base `GraphDefinition` via `merge_graph_definitions`: **local definitions always win** —
  `graph_attrs` fields only fill in if still at their default, `node_types`/`icons`/
  `node_templates`/`groups` are deduplicated by id (an existing local `node_type` with a
  blank `func` still inherits the imported one's script), `layout` only fills in if the
  base has none, and `graph` instances merge by `name`.
- `import: [ids...]` filters which `node_types` a given import contributes; `as: alias`
  renames the (single, if `import` names exactly one type) imported type's `id` to `alias`,
  or prefixes multiple (`alias_<original_id>`) — see `test_selective_import_and_aliasing`
  in `imports.rs`'s test module for the exact renaming rule.

### Layout engine & group boxes (`src/systems/layout.rs`, `src/systems/group_overlay.rs`)

`compute_graph_layout` is a pure function (`GraphDefinition -> ComputedLayout`, no ECS/Bevy
state) that assigns every node an `(x, y)` from `graph_defn.layout: Option<LayoutConfig>`
(`LayoutType::Manual | Circular | Grid | Hierarchical`, default `Manual`) — called both by
`node_system::create_nodes` for initial placement and, wherever it's re-run, to re-derive
group bounding boxes. A `NodeConnection.pos: Option<[f32; 2]>` always wins outright
regardless of layout mode (an explicit per-node pin); `.offset` nudges the computed position
instead of replacing it.
- **Hierarchical** (`compute_hierarchical_layout`) is the real layout engine: nodes are
  first collapsed into `MacroElement`s — either a standalone node or an entire `GroupDef`
  (from `graph_defn.groups`, membership via `GroupDef.nodes` or a node's own `group:`
  field) treated as one macro-box — then ranked by longest-path topological layering over
  macro-to-macro edges (derived from `links`, cycles fall back to incremental ranks so a
  cyclic graph still lays out instead of hanging), ordered within a rank by explicit
  `order`/`rank` overrides or macro index, and finally projected into world space along
  `LayoutConfig.direction` (`Lr`/`Rl`/`Tb`/`Bt`) with spacing from `get_rank_sep()`/
  `get_node_sep()`/`get_group_sep()` (defaults 260/140/320, each backed by a
  `rank_sep`/`node_sep`/`group_sep` field with a long list of `serde(alias = ...)` spellings
  — `rank_spacing`, `rankSep`, `rank-sep`, etc. — so hand-written YAML doesn't have to guess
  the canonical name). A group's own internal layout direction defaults to the *opposite*
  of the global direction (an `Lr` pipeline stacks its groups' members `Tb`) unless
  `GroupDef.layout.direction` overrides it, and can set its own member spacing via
  `GroupLayoutConfig.sep`/`get_spacing`.
- **Circular**/**Grid** are simpler single-pass placements (ellipse by index, or a
  `sqrt(n)`-column matrix); both still honor an explicit `.pos`/`.offset` per node.
- `ComputedLayout.group_boxes: Vec<GroupBoxBounds>` (only populated by the hierarchical
  path) is what `group_overlay::update_group_boxes` renders: on `GraphChange`, it
  despawn-recursive's every existing `GroupBoxMarker` entity and respawns one lyon
  `RoundedPolygon` (+ an optional uppercased title `Text2d`) per box, at `GROUP_BOX_Z = 2.0`
  — behind connectors (z=10) and nodes (z=100+) but in front of the background grid (z=0).
  Box visibility/style come from `GroupDef.style: Option<GroupStyle>` (`r#box: Some(false)`
  skips rendering the box entirely — useful for a group used only to drive layout grouping,
  not a visible container — plus `radius`/`border_width`/`bg`/`border`/`padding`).

### Runtime tick loop (`src/systems/rhai_engine.rs`)

Every frame, `execute_rhai_engine`:
1. Ticks each node's `Timer`; when it fires and the node has a compiled script, calls
   `on_timer(context)` with the node's params pushed into scope and its persisted state
   bound as `state`.
2. Drains each connector's `msg_delivered` queue and calls `on_msg(msg)` on the receiving
   node for each delivered message.
3. Rhai scripts call the registered host functions (all `register_fn`'d in
   `rhai_engine.rs`, ~line 515 on) to talk back to the engine: `log(s)` — three
   overloads (`String`, `Dynamic`, and a 2-arg `Dynamic, Dynamic` that space-joins —
   `log(a, b)`) all funnel into a thread-local `log_store` drained after the handler and
   forwarded via `log_dsa_event!`; `send(to, msg)` (pushes into a thread-local message
   store — `send_messages` fans it out to the `Messages` component on whichever
   `NodeConnector` links the two node names, becoming an in-flight `Message` with a 3s
   timer); `draw(shapes)` (see below); `random_chance(percent)` (returns `true` with
   roughly that % probability, for scripts simulating flaky/failing behavior without a
   manually-toggled param); `random_int(min, max)` (returns an integer in `[min, max)`,
   e.g. `random_int(0, links.len())` to pick a random peer — degenerate `max <= min`
   returns `min` instead of panicking, so a node with no links doesn't crash calling it);
   `spawn_node`/`despawn`/`link`/`unlink` (see the topology-changes section below); and
   `explain(...)` — four overloads: `explain(key, text)`, `explain(key, text, opts)`,
   and two auto-keying shorthands, `explain(text)`/`explain(text, opts)`, that reuse
   `text` itself as the dedup `key` (`opts` is accepted but currently unused/ignored in
   all three variants that take it).
4. `state` — a Rhai scope variable bound to the node's persistent `Dynamic` field
   (`Node::state`) — round-trips across calls — this is how a node keeps memory between
   ticks/messages (`state.count += 1`, etc.). Both call sites (`on_timer` and `on_msg`,
   plus `init_scope` for `on_init`) move it into/out of the `Scope` rather than cloning
   it - `scope.push_dynamic("state", std::mem::take(&mut node.state))` going in,
   `node.state = scope.remove::<Dynamic>("state").unwrap_or_default()` coming out. This
   matters because `Dynamic::clone` is a *deep* clone (it recurses into every
   string/array/map a node's state contains), while a move is O(1) regardless of what's
   inside - `benches/rhai_state_roundtrip.rs` (`cargo bench`) measures the old clone-based
   path against this one side by side and shows the move-based round-trip alone running
   3.6-4.7x faster (more for bigger state), a 26-39% reduction in total per-tick call cost.
   Safe to do because the entry doesn't need to survive the call either way - the next line,
   `scope.rewind(init_size)`, discards it regardless.
   **Earlier regression (fixed):** for a while, both call sites pushed *two* scope
   variables - `globals` and `state` - seeded from the same clone, then guessed which one
   to keep afterward by checking whether `state` looked like a non-empty map. That heuristic
   only worked on the very first state-mutating call; after that, `state` was always a
   non-empty map regardless of whether the script touched it (globals-writing scripts
   never wrote through `state`, so it just kept holding the stale pre-call snapshot), so
   the heuristic kept "winning" with stale data and silently discarding every subsequent
   `globals.x = ...` mutation. Scripts written against `state.x = ...` were unaffected
   (that's the variable actually being written), which is why some tutorial examples
   looked fine and others (anything using `globals`) appeared permanently stuck after
   their first tick. Fixed by removing the `globals` binding entirely - `state` is now the
   only name, so there's nothing to disambiguate.
5. The Rhai compile/eval engines (`parse_graph2` and `initialize_engine`) both raise
   `set_max_expr_depths` to 256 — the default in-function limit of 32 is too low for
   realistic handlers (e.g. a map literal containing an inline `if`).

### Custom node overlays (`draw()` — `src/parser/draw.rs`, `src/systems/node_overlay.rs`)

A handler can call `draw([ #{shape:"rect", ...}, #{shape:"text", ...}, ... ])` to paint a
custom overlay on its node. Flow mirrors `send()`: `draw()` writes the shape list into a
per-`execute_rhai_engine` thread-local (`draw_store`), which is drained right after each
handler into `Node::overlay` + `Node::overlay_dirty`. `node_overlay::render_node_overlays`
then despawn-and-respawns that node's `NodeOverlayShape` child entities (lyon shapes +
`Text2d`, parented to the node so they track drag/pan/zoom). `draw()` *replaces* the
overlay; not calling it leaves the last one; `draw([])` clears. Supported shapes: `rect`
(optional `radius`), `circle`, `line`, `polygon`/`polyline`, `text`; coords are node-local,
y-up, capped at `MAX_SHAPES_PER_NODE`. Rhai functions can't see script-level vars, so a
helper called from a handler must take what it needs as params. Color/size keys accept
a couple of natural aliases (`paint_of`/`color_of_aliases`/`num_aliases` in `draw.rs`) so
a shape map doesn't have to match one exact vocabulary: `fill` also accepts `bg`/`color`,
`stroke` also accepts `border`/`border_color` (`color` for `line`), `stroke_width` also
accepts `border_width`/`width`, and a `text` shape's `size` also accepts `font_size` -
`fill`/`stroke`/`stroke_width`/`size` are still the canonical names and win if both are
present. `draw()` also works from `on_init`, not just `on_timer`/`on_msg` - `parse_graph2`
builds its own draw_store-backed engine (`create_rhai_engine`) so the very first `on_init`
call already has a real, working `draw()` (`log`/`send`/`spawn_node`/etc are still
harmless no-ops there, since there's no live ECS world yet to affect).

### Runtime topology changes (`spawn_node()`/`despawn()`/`link()`/`unlink()`)

A handler can grow or reshape the running graph without touching the YAML/editor at
all: `spawn_node(name, node_type, links)` creates a new node instance (its `on_init`
runs immediately, through the live engine, so `draw()`/`log()` in it work); `despawn(name)`
removes any node by name, including the caller; `link(peer)`/`unlink(peer)` add/remove
`peer` from the *calling* node's own `links` (self-scoped by convention, not enforcement).
(Named `spawn_node` rather than `spawn` because Rhai reserves the bare word `spawn`,
presumably for a future async/threading feature - it's a parse error under any name.)

Mechanism: same thread-local-queue-drained-after-the-handler pattern as `send`/`draw`.
`link`/`unlink` apply immediately to the calling `Node` (safe - no reallocation of
`node_instances` involved); `spawn_node`/`despawn` are attributed to the caller and
queued, then applied once at the very end of `execute_rhai_engine`, after every node's
`&mut Node` borrow from `node_map` has been dropped (`node_instances.push`/`retain`
would otherwise invalidate those borrows mid-iteration). Since `links` was pushed as a
Rhai *constant* in `init_scope`, updating it from the host can't use `Scope::set_value`
(panics on a read-only entry) - it uses `Scope::remove` + `push_constant` instead;
`remove` doesn't check access mode because that check lives in the interpreter's eval
path (assignment/non-pure-method-call sites), not in the `Scope` struct itself.

Rendering reacts via two new events distinct from the YAML-edit `GraphChange` (which
still does a full despawn/respawn of every node - deliberately, since a YAML edit may
have changed anything and there's no state worth preserving across it): `NodeAdded`/
`NodeRemoved` (`resources/graph_def.rs`) tell `node_system::create_nodes` to spawn/despawn
only that one entity, leaving every other node's position, drag state and overlay
untouched. `update_connectors` was rewritten from "despawn all connectors whenever any
node is added" to a real add/remove diff against the currently-declared edges (now
read from live `node_instances[].links`, not the YAML-shaped `graph_defn.graph`, so
`link()`/`unlink()` only have one copy to update) - this also fixed a pre-existing bug
where adding a single node via a YAML edit destroyed every other connector's in-flight
`Messages` queue. `node_pulse::pulse_on_tick` uses `try_insert` (not `insert`) for the
same reason: a node's last `NodeTicked` (pulse animation) and its own same-tick
`despawn()` race to be the command that lands first when the schedule flushes.
`MAX_NODES` (200) caps a runaway `spawn_node()` loop. See
`web/static/examples/cell_division.yml` for a full spawn/link/unlink/despawn demo.

### Narration bubbles (`explain()` — `src/systems/explain_bubble.rs`, `src/systems/radial_blur.rs`, `src/resources/narration.rs`)

A handler can call `explain(key, text)` to show a one-time, pausing, animated narration
bubble anchored to the calling node — for first-occurrence "here's what's happening" beats
in a simulation (see `web/static/examples/two_phase_commit.yml`). `key` is an
author-chosen dedup id (e.g. `"first-prepare"`): `PendingExplain.seen: HashSet<String>`
means each key only ever shows once per graph load, no matter how many times the handler
calls `explain()` with it again (e.g. on every tx). Same thread-local-queue pattern as
`send`/`draw`: `explain()` pushes an `ExplainEntry { node_name, key, text }` into
`PendingExplain.queue`, drained one at a time by `show_next_explain`.

- `show_next_explain` (runs every frame while idle): pops the queue, calls
  `sim_time.pause()` (`Time<Virtual>::pause()` — freezes node timers, in-flight message
  animation, and any `Time`-driven tween all at once), and spawns the bubble: a body +
  pointer-triangle (aimed at the anchor node, built via `node_overlay::spawn_shape` — the
  same shape-drawing code `draw()` overlays use), word-wrapped text, a decorative Continue
  button, and — layered on top of it — a fully transparent `SpriteBundle` "hit region"
  carrying the actual `On::<Pointer<Click>>::run(dismiss_explain_bubble)` handler. The hit
  region is necessary because `bevy_mod_picking` here is built
  `features = ["backend_sprite"]` only — lyon `Mesh2d` shapes (the button's visual) are
  never pickable in this app, so anything meant to be clicked needs an invisible `Sprite`
  on top of it.
- Bubble pop-in is a hand-rolled `BubblePop` component + `animate_bubble_pop` system driven
  by `Time<Real>`, **not** `bevy_tweening::Animator` — `Animator` reads the generic `Time`
  resource (which mirrors `Time<Virtual>`), so a tween started the same frame the sim
  pauses would see `delta == 0` forever and never animate. Anything that must keep moving
  while the sim is paused (bubble pop-in, the backdrop, the blur fade below) has to be
  driven by `Time<Real>` explicitly.
- `dismiss_explain_bubble` (the Continue click handler) despawns the bubble and unpauses
  `sim_time` only once `PendingExplain.queue` is empty — otherwise it immediately shows the
  next queued entry, still paused.
- `GraphChange` (any YAML edit) calls `PendingExplain::reset()` (clears `seen` and `queue`)
  and unpauses, so a fresh graph load never starts frozen on a stale bubble.

**Radial blur (`radial_blur.rs`)**: while any bubble is up, a `RadialBlurSettings.intensity`
uniform fades in/out (`animate_radial_blur`, `Time<Real>`-driven, same pause-safety reason
as above) and drives a custom WGSL post-process (`radial_blur.wgsl`, embedded via
`load_internal_asset!` so there's no wasm asset-path to resolve) — a `ViewNode` spliced into
the `Core2d` render graph between `Node2d::Tonemapping` and `Node2d::EndMainPassPostProcessing`,
multi-sampling the already-rendered frame toward its center and averaging (deliberately
color-only, no depth texture — this app's WebGL2 target already can't do real depth-of-field,
see the "Disabling depth of field" log line from `bevy_core_pipeline`).

Because that post-process runs on the whole `Core2d` pass, the bubble would get blurred too
if it were part of the same render — so it isn't. A second camera (`BubbleCamera` in
`main.rs`'s `setup_camera`, marked by `components::camera::BubbleCamera`) composites on top:
`order: 1`, `clear_color: ClearColorConfig::None`, renders only `RenderLayers::layer(1)`, and
carries no `RadialBlurSettings` (the post-process plugin only touches views that have the
component). Every bubble/backdrop entity is tagged `RenderLayers::layer(1)` so only this
camera draws them, crisp, over the (possibly blurred) world. `sync_bubble_camera` copies the
main camera's `Transform`/`OrthographicProjection` onto it every frame, since `bevy_pancam`'s
`PanCam` only drives the entity it's attached to (the main camera) and the bubble camera has
no other way to track pan/zoom.

**Gotcha this already caused once**: any query meant to uniquely match "the" camera by
`With<Camera2d>` alone now matches both cameras and `.single()`/`.single_mut()` on it will
panic — which, since an uncaught wasm panic halts the whole Bevy app loop, looks exactly
like a freeze/deadlock from the user's perspective (this broke node dragging — `drag.rs`'s
projection lookup — the first time `BubbleCamera` was added; fixed by adding
`Without<BubbleCamera>`). Any future second-camera-agnostic query needs the same filter.

### Connectors (`src/systems/update_connectors.rs`, `src/components/node_connector.rs`)

A `NodeConnector` is drawn as a cubic bezier between two node centers, inset so it
visibly stops just outside each icon's edge rather than running into it. The curve bows
*perpendicular* to the A→B line (magnitude proportional to distance, clamped in
`bow_amount`) rather than toward a fixed diagonal offset — a fixed offset could bow the
"wrong" way depending on how two nodes happened to be arranged; deriving it from the line
itself keeps the shape consistent no matter the layout. `connector_geometry`/
`connector_stroke` are shared by both the initial spawn (`generate_line`) and the
per-frame retrace loop so newly-created and moving connectors can't drift out of sync
with each other.

Connectors used to have a hover highlight (an invisible `ConnectorHitRegion` sprite child
per connector carrying `On::<Pointer<Over>>`/`<Out>` handlers, driving `NodeConnector.hovered`
in `update_connector_style`) - removed for performance. `bevy_mod_picking`'s sprite backend
hit-tests and z-sorts *every* pickable `Sprite` on *every* frame regardless of pointer
movement or button state (hover has to be recomputed continuously since what's under the
pointer can change with no input at all - e.g. a message bubble animating under a
stationary cursor), so one extra sprite per connector was a real, scaling cost: at 500
nodes (~1500 edges) this alone was worth roughly 2x the frame rate (see
`web/static/examples/random_mesh_200.yml`-scale measurements) once profiling ruled out
everything else as the bottleneck. Node click/drag/selection still work (unaffected -
they hit-test node icon sprites directly, not a connector's).

**Live styling** (`update_connector_style`, blends the user-configured `connection_color`
toward an indigo `ACCENT_COLOR` rather than replacing it, so a custom base color still
shows through):
- **Traffic**: width/accent scale with `Messages.msg_inflight.len()` — a connector
  currently carrying messages reads visibly thicker/brighter than an idle one.
- **Delivery flash**: `NodeConnector.flash` is set to `1.0` in `update_message.rs` when a
  message lands (moves from `msg_inflight` to `msg_delivered`) and decays back to `0` over
  `FLASH_DECAY_SECS`, giving a brief highlight on the edge itself, not just the arriving
  message icon.
- **Selection adjacency**: connectors touching the currently-`SelectedNodeMarker`'d node
  get a persistent highlight bump, computed by checking `conn.id1`/`id2` against the
  selected node's name each frame.

### Ingestion / state machine (`src/systems/ingest_code.rs`, `main.rs`)

- `compile_code` and `get_code_schema` are the two `#[wasm_bindgen]` entry points the SvelteKit
  frontend calls directly (see `web/src/routes/+page.svelte`). `compile_code` just validates +
  stashes the YAML string in a `lazy_static` `Mutex<String>` (`E_CODE`); it does not mutate the
  live ECS world directly (wasm/JS calls happen outside the Bevy schedule).
  `ingest_codechange` (a normal Bevy system) polls that global each frame, and on change
  re-parses it and swaps in the new `GraphDefinitionRes`, flipping `LoadingState` to `Loading`.
- `LoadingState` (`Loading` / `Ready`, in `resources/common_assets.rs`) gates almost every other
  system via `run_if(resource_equals(...))` in `main.rs` — asset loading (`resource_loader`)
  only runs while `Loading`; simulation/rendering systems only run while `Ready`. When all
  fonts/icons finish loading, `resource_loader::load_assets` flips state back to `Ready` and
  fires a `GraphChange` event, which `node_system::create_nodes` uses to despawn and redraw all
  node entities from scratch.
- Node/connector visuals are rebuilt reactively from `GraphDefinitionRes`, not incrementally
  patched — `create_nodes` and `update_connectors` both do despawn-and-respawn-all on relevant
  changes rather than diffing.

### Rendering/interaction systems (`src/systems/`)

- `node_system.rs` spawns node entities (icon + label sprites, tween-animated into a random
  position, `bevy_mod_picking` click/drag handlers).
- `update_connectors.rs` — see the dedicated section above.
- `update_message.rs` walks each connector's path (`lyon_algorithms::walk`) to animate an
  in-flight message icon/label along the curve based on its timer's progress, then moves it
  from `msg_inflight` to `msg_delivered` once the timer finishes (picked up by
  `rhai_engine::execute_rhai_engine` next frame to trigger `on_msg`).
- `zoom_panel.rs` / `drag.rs` handle Ctrl/Cmd-scroll zoom and node dragging.
- `node_progress.rs`'s `update_tick_progress` drives per-node progress-bar overlays
  (`TickProgressFill` components: `Bar`/`Ring`/`Pie`/`Segmented` kinds) off
  `node.timer.fraction_remaining()` each frame — a purely visual "time until next tick"
  indicator, independent of `draw()`'s overlay shapes.
- `explain_bubble.rs` / `radial_blur.rs` implement the `explain()` narration-bubble system —
  see the dedicated section above.
- `browser_resize.rs` (wasm-only, `#[cfg(target_arch = "wasm32")]` in `systems/mod.rs`) keeps
  the Bevy canvas sized to its parent `<div>`.
- `ui.rs`'s `graph_properties_viewer` is the app's one `bevy_egui` window ("Graph
  Properties" — sim-speed slider, background/text/connection color pickers, and per-node
  param editing when a node is selected). `.collapsible(true).default_open(false)` so it
  loads collapsed to just its title bar instead of covering the canvas on every load.

### wasm/JS bridge (`src/wasm/`)

`c_log!` forwards to `console.log`. `log_dsa_event!` is the channel Rhai's `log()` calls ride
on: it dispatches a DOM `CustomEvent("dsa-log-event")` on `#dsa-log-event-listener`, which
`+page.svelte` listens for and feeds into the log panel (`panels.ts`) — this is the only path
from inside a running simulation back into the frontend's UI log, so grep for
`log_dsa_event`/`dsa-log-event` when tracing "why doesn't my log show up" issues.

### Frontend (`web/src/routes/`)

- `+page.svelte` is the app shell: loads the wasm module (`init()` from `./dsa` — the
  symlinked wasm-bindgen output), wires up the Menubar, dockview panel layout (`panels.ts`),
  and the log event listener described above.
- `Monaco.svelte` wraps `monaco-editor` + `monaco-yaml`, configured with the JSON Schema
  fetched from `get_code_schema()` so the YAML editor validates/autocompletes against the
  Rust-side `File`/`GraphDefinition` structs.
- `panels.ts` builds the `dockview-core` layout (editor, canvas, log table via
  `tabulator-tables`).
- Styling is Tailwind + daisyUI (`tailwind.config.js`, `app.css`).

### Tutorial / Learn panel (`web/static/tutorial/`, `web/src/routes/tutorial.ts`, `Help.svelte`, `src/systems/native_examples.rs`)

`tutorial.ts` is a static, hand-maintained index of chapters/lessons
(`TutorialChapter { id, title, dir, lessons: TutorialLesson[] }`) — each lesson pairs a
markdown file with an optional runnable YAML snippet, both fetched at runtime from
`static/tutorial/<dir>/` (mirrors the older Examples menu's `fetch()`-from-`static/examples/`
pattern, not bundled into the wasm/JS build). Currently five chapters: ch1 Fundamentals,
ch2 Layout & Architecture, ch3 Custom Visuals & Node Overlays, ch4 Theming & Visual Styling,
ch5 Distributed Algorithms & plibs (imports/presets). `Help.svelte` renders the two-level
chapter/lesson nav plus the markdown (via `carta-md`, `sanitizer: false` — safe only because
content always comes from this repo's own static files, never user/network input) and a
"Load this example" button that calls back into `+page.svelte`'s example-loading path
(same one `panels.ts` wires the old Examples menu to) with `<dir>/<yaml>`.
- **Native parity**: a plain `cargo run` has no browser/Monaco/Learn panel at all, so
  `native_examples.rs`'s `examples_picker` (a `bevy_egui` window, collapsed by default like
  `ui::graph_properties_viewer`) is a flattened developer-convenience mirror — one button
  per lesson that has YAML, embedding each file via `include_str!` (so the native binary
  doesn't depend on the process's working directory) and running it through the same
  `compile_code()` path the browser uses. Its `EXAMPLES` list has to be kept in sync with
  `tutorial.ts` by hand (there's no shared source of truth between Rust and TS here); a
  lesson with no YAML (e.g. ch1's intro) is omitted from both, same reasoning. `DEFAULT_EXAMPLE`
  (`EXAMPLES[0].1`) is also what `load_native_demo_on_startup` seeds the native app with.

### Desktop shell (`web/src-tauri/`)

A Tauri v2 wrapper around the *same* SvelteKit + wasm build the browser serves - not a
separate UI, just that static site (`frontendDist` in `tauri.conf.json` points at
`../../package/release`, the same `adapter-static` output `cargo make build-all` produces)
loaded into a native webview instead of a browser tab. This is distinct from the plain
native `cargo run` binary (`src/main.rs`) described above, which renders the Bevy canvas
directly with no webview/JS/Svelte layer at all - the Tauri shell exists to ship the actual
web UI (Monaco editor, dockview panels, log table and all) as a installable desktop app.

- `cargo make tauri-dev` / `cargo make -p production tauri-build` build the wasm module
  first (`build-wasm-module`/`build-all`), then hand off to `npm run tauri dev`/`build` -
  necessary because Tauri's own `beforeDevCommand`/`beforeBuildCommand` just run
  `npm run dev`/`build` directly, which - like a bare `npm run dev` in `web/` - can't
  resolve the symlinked wasm import (`web/src/routes/dsa.js`) on a clean checkout without
  the wasm side having been built at least once first.
- The Rust crate here (`procsim-tauri` / lib `procsim_tauri_lib`) is a separate, independent
  crate from the root `dsa` package - no `[workspace]` ties them together, so building one
  doesn't pull in or need the other's dependencies (in particular, it never touches
  `wasm32-unknown-unknown` or Bevy at all).
- `tauri build` output goes to `web/src-tauri/target/release/bundle/` (`.app`/`.dmg` on
  macOS) - gitignored the same way `/target` and `/package` are.
