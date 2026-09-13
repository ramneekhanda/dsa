//! Native-only "Tutorial" picker. There's no browser/Monaco editor (or Learn
//! panel) in a native `cargo run` (see `ingest_code::load_native_demo_on_startup`'s
//! doc comment), so there's also nowhere to click a tutorial lesson the way
//! the SvelteKit frontend's Learn panel works. This is a small `bevy_egui`
//! window (the same UI toolkit `ui::graph_properties_viewer` already uses)
//! listing every lesson's runnable YAML under `web/static/tutorial/`,
//! embedded at compile time via `include_str!` so the native binary is
//! self-contained and doesn't depend on the process's working directory.
//! Clicking one runs it through the exact same `compile_code()` path the
//! browser uses. A lesson with no accompanying YAML (chapter 1's
//! introduction) isn't listed here - same reasoning as the Learn panel only
//! showing a "Load this example" button for lessons that have one.

use bevy_egui::{egui, EguiContexts};

use crate::systems::ingest_code::compile_code;

/// (menu label, embedded YAML text) - mirrors `tutorial.ts`'s chapter/lesson
/// list, flattened (native has no room for the Learn panel's two-level nav
/// or its prose) since this is a developer convenience, not user-facing
/// polish.
const EXAMPLES: &[(&str, &str)] = &[
    (
        "1.2 Your First Graph",
        include_str!("../../web/static/tutorial/ch1/02_first_graph.yml"),
    ),
    (
        "1.3 Nodes Talking To Each Other",
        include_str!("../../web/static/tutorial/ch1/03_messaging.yml"),
    ),
    (
        "1.4 Configurable Node Types",
        include_str!("../../web/static/tutorial/ch1/04_params_and_icons.yml"),
    ),
    (
        "1.5 State and Logging",
        include_str!("../../web/static/tutorial/ch1/05_state_and_logging.yml"),
    ),
    (
        "1.6 Putting It Together",
        include_str!("../../web/static/tutorial/ch1/06_putting_it_together.yml"),
    ),
    (
        "1.7 Multilevel Routing",
        include_str!("../../web/static/tutorial/ch1/07_multilevel_routing.yml"),
    ),
    (
        "1.8 Constants & State Persistence",
        include_str!("../../web/static/tutorial/ch1/08_constants_and_scope.yml"),
    ),
    (
        "1.9 Interactive Narration (explain)",
        include_str!("../../web/static/tutorial/ch1/09_interactive_narration.yml"),
    ),
    (
        "2.1 draw() Basics",
        include_str!("../../web/static/tutorial/ch2/01_draw_basics.yml"),
    ),
    (
        "2.2 Shape Primitives",
        include_str!("../../web/static/tutorial/ch2/02_shapes.yml"),
    ),
    (
        "2.3 Reusable Node Templates",
        include_str!("../../web/static/tutorial/ch2/03_node_templates.yml"),
    ),
    (
        "2.4 Parametrized Templates",
        include_str!("../../web/static/tutorial/ch2/04_parametrized_templates.yml"),
    ),
    (
        "3.1 Cloud Architecture Theme",
        include_str!("../../web/static/tutorial/ch3/01_cloud_cards.yml"),
    ),
    (
        "3.2 Datacenter Rack Units",
        include_str!("../../web/static/tutorial/ch3/02_datacenter_rack.yml"),
    ),
    (
        "3.3 Cyberpunk Neon HUD",
        include_str!("../../web/static/tutorial/ch3/03_cyberpunk_hud.yml"),
    ),
    (
        "3.4 Minimal Capsule Pills",
        include_str!("../../web/static/tutorial/ch3/04_capsule_pills.yml"),
    ),
    (
        "3.5 Layered & Composed Themes",
        include_str!("../../web/static/tutorial/ch3/05_layered_theming.yml"),
    ),
    (
        "3.6 Remote Imports & Presets",
        include_str!("../../web/static/tutorial/ch3/06_remote_imports.yml"),
    ),
    (
        "3.7 Remote Theme Packages",
        include_str!("../../web/static/tutorial/ch3/07_remote_theme_imports.yml"),
    ),
];

/// Same list `load_native_demo_on_startup` seeds `E_CODE` with at `Startup`
/// - kept as a named constant so the two stay in sync by construction rather
/// than by remembering to update both places.
pub const DEFAULT_EXAMPLE: &str = EXAMPLES[4].1; // "1.6 Putting It Together"

pub fn examples_picker(mut contexts: EguiContexts) {
    // Starts collapsed, same reasoning as ui::graph_properties_viewer's
    // Graph Properties window - a native window shouldn't open with a panel
    // already covering the canvas.
    egui::Window::new("Tutorial")
        .collapsible(true)
        .default_open(false)
        .show(contexts.ctx_mut(), |ui| {
            for (label, yaml) in EXAMPLES.iter() {
                if ui.button(*label).clicked() {
                    let result = compile_code(yaml.to_string());
                    if !result.result() {
                        eprintln!("failed to load example {label}: {}", result.error_log());
                    }
                }
            }
        });
}
