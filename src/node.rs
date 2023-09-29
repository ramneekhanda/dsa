use crate::ui::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;
use std::collections::HashMap;
use rustpython_vm::Interpreter as py_interpreter;

type NodeDepsMap = HashMap::<String, Vec<String>>;
#[derive(Component)]
pub struct Node {

}


pub struct NodeBundle {

}
pub fn layout_changed(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  asset_server: Res<AssetServer>,
  graph_defn: Res<GraphDefinition>,
  query: Query<Entity, With<Node>>,
) {
  if graph_defn.is_changed() {
    for entity in query.iter() {
      commands.entity(entity).despawn();
    }
    for node in graph_defn.graph.keys() {
      spawn_node(node, &mut commands, &asset_server, &mut meshes, &mut materials);
    }
  }
}

fn spawn_node(node_name: &String,
              commands: &mut Commands,
              asset_server: &Res<AssetServer>,
              meshes: &mut ResMut<Assets<Mesh>>,
              mut materials: &mut ResMut<Assets<ColorMaterial>>,
) {
  let font = asset_server.load("fonts/FiraSans-Bold.ttf");
  let text_style = TextStyle {
    font: font.clone(),
    font_size: 60.0,
    color: Color::WHITE
  };
  let text_alignment = TextAlignment::Center;

  let mut rng = rand::thread_rng();
  let x = rng.gen_range(-1000.0..1000.0);
  let y = rng.gen_range(-1000.0..1000.0);
  commands.spawn((
    MaterialMesh2dBundle {
      mesh: meshes.add(shape::Circle::new(20.).into()).into(),
      material: materials.add(ColorMaterial::from(Color::PURPLE)),
      transform: Transform::from_translation(Vec3::new(x, y, 0.)),
      ..default()
    },

    Text2dBundle {
      text: Text::from_section("Hi", text_style).with_alignment(text_alignment),
      ..default()
    },

    Node{}
  ));

}
