mod node;
mod ui;
use std::collections::HashMap;
use bevy_tweening::TweeningPlugin;

use bevy::{prelude::*, window::WindowMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use ui::{CodeStorage, GraphDefinition};

fn main() {
  App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        mode: WindowMode::BorderlessFullscreen,
        fit_canvas_to_parent: true,
        ..default()
      }),
      ..default()
    }))
    .add_plugins(WorldInspectorPlugin::new())
    .add_plugins(TweeningPlugin)
    .insert_resource(CodeStorage{
      code: String::new(),
      graph_code: String::new(),
      console: String::new(),
    })
    .insert_resource(GraphDefinition {
      graph: HashMap::new(),
    })
    //.add_plugins(EguiPlugin)
    .add_systems(Startup, setup_camera)
    .add_systems(Update, (ui::setup_ui, node::update_connectors, node::update_nodes.after(node::update_connectors)))
    .run();
}

fn setup_camera(
  mut commands: Commands
) {
  commands.spawn(Camera2dBundle{
    ..Default::default()
  });
}
