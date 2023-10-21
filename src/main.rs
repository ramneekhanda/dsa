mod ui;
mod actors;
mod shimmer;
mod python_interp;

mod components;
mod systems;
mod parser;
use std::collections::HashMap;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_tweening::TweeningPlugin;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_mod_picking::prelude::*;
use std::time::Duration;

use ui::{CodeStorage, GraphDefinition};

fn main() {
  App::new()
    .add_plugins(DefaultPlugins.set(low_latency_window_plugin()))
    .add_plugins(WorldInspectorPlugin::default())
    //.add_plugins(EguiPlugin)
    .insert_resource(Msaa::Sample8)
    .add_plugins(DefaultPickingPlugins)
    .add_plugins(ShapePlugin)
    .add_plugins(TweeningPlugin)
    .insert_resource(ui::UiState::new())
    .insert_resource(systems::demo_message::DemoTimer{
      timer: Timer::new(Duration::from_secs(4), TimerMode::Repeating),
    })
    .insert_resource(CodeStorage{
      code: String::new(),
      console: String::new(),
    })
    .insert_resource(GraphDefinition {
      graph: HashMap::new(),
    })
    .add_systems(Startup, setup_camera)
    .add_systems(Update, (systems::demo_message::demo_send_message,
                          systems::update_message::update_message_path))
    .add_systems(Update, (ui::show_ui_system, systems::update_connectors::update_connectors, systems::update_node::update_nodes))
    .run();
}

fn setup_camera(
  mut commands: Commands,
) {
  commands.spawn(Camera2dBundle{
    ..Default::default()
  });
}
