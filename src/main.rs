mod node;
mod ui;
mod actors;
mod shimmer;

use std::collections::HashMap;
use bevy_tweening::TweeningPlugin;
use bevy_egui::EguiPlugin;
use bevy::{prelude::*, window::WindowMode};
use ui::{CodeStorage, GraphDefinition};
use shimmer::*;

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
    .add_plugins(bevy_screen_diags::ScreenDiagsTextPlugin)
    .add_plugins(TweeningPlugin)
    .insert_resource(ui::UiState::new())
    .insert_resource(CodeStorage{
      code: String::new(),
      console: String::new(),
    })
    .insert_resource(GraphDefinition {
      graph: HashMap::new(),
    })
    .add_plugins(EguiPlugin)
    .add_systems(Startup, setup_camera)
    .add_systems(Update, (ui::show_ui_system, node::update_connectors, node::update_nodes.after(node::update_connectors)))
    .run();
}

fn setup_camera(
  mut commands: Commands
) {
  commands.spawn(Camera2dBundle{
    ..Default::default()
  });
}
