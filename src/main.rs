mod ui;
mod shimmer;
mod python_interp;

mod components;
mod systems;
mod parser;
mod python;
use bevy_egui::EguiPlugin;
//use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_tweening::TweeningPlugin;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_mod_picking::prelude::*;
use std::time::Duration;
use parser::graphv2::GraphDefinition;
use ui::{CodeStorage, GraphDefinitionRes};

fn main() {
  App::new()
    .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
    .add_plugins(DefaultPlugins)
    //.add_plugins(WorldInspectorPlugin::default())
    .add_plugins(EguiPlugin)
    .insert_resource(Msaa::Sample4)
    .add_plugins(DefaultPickingPlugins)
    .add_plugins(ShapePlugin)
    .add_plugins(TweeningPlugin)
    .insert_resource(systems::demo_message::DemoTimer{
      timer: Timer::new(Duration::from_secs(4), TimerMode::Repeating),
    })
    .insert_resource(CodeStorage{
      code: String::new(),
      console: String::new(),
    })
    .insert_resource(GraphDefinitionRes {
        graph_defn: GraphDefinition::default()
    })
    .add_systems(Startup, setup_camera)
    .add_systems(Update, (systems::demo_message::demo_send_message,
                          systems::update_message::update_message_path))
    .add_systems(Update, (systems::background::update_background, ui::draw_codeviewer, systems::update_connectors::update_connectors, systems::update_node::update_nodes))
    .run();
}

fn setup_camera(
  mut commands: Commands,
) {
  commands.spawn(Camera2dBundle{
    ..Default::default()
  });
}
