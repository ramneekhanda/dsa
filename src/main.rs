mod node;
mod ui;
mod actors;
mod shimmer;
mod python_interp;

use std::collections::HashMap;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_tweening::TweeningPlugin;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_mod_picking::prelude::*;


use ui::{CodeStorage, GraphDefinition};

fn main() {
  App::new()
    .add_plugins(DefaultPlugins.set(low_latency_window_plugin()))
    //.add_plugins(WorldInspectorPlugin::default())
    .add_plugins(EguiPlugin)
    .insert_resource(Msaa::Sample8)
    .add_plugins(DefaultPickingPlugins)
    .add_plugins(ShapePlugin)
    .add_plugins(TweeningPlugin)
    .insert_resource(ui::UiState::new())
    .insert_resource(CodeStorage{
      code: String::new(),
      console: String::new(),
    })
    .insert_resource(GraphDefinition {
      graph: HashMap::new(),
    })
    .add_systems(Startup, setup_camera)
    .add_systems(Update, (ui::show_ui_system, node::update_connectors, node::update_nodes, node::show_dashboard))
    .run();
}

fn setup_camera(
  mut commands: Commands,
) {
  commands.spawn(Camera2dBundle{
    ..Default::default()
  });
}
