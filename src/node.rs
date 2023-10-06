use crate::ui::*;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_tweening::{*, lens::*};
use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::time::*;
use bevy_prototype_lyon::prelude::*;
use bevy_mod_picking::prelude::*;

#[derive(Component, Debug)]
pub struct Node {
  node_text: String,
}

#[derive(Component, Debug)]
pub struct Connector {
  _conn_text: String,
}

impl Default for Node {
  fn default() -> Self {
    Node {
      node_text: "ANODE".to_string()
    }
  }
}

pub fn update_nodes(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  graph_defn: Res<GraphDefinition>,
  query: Query<Entity, With<Node>>,
) {
  if graph_defn.is_changed() {
    for entity in query.iter() {
      commands.entity(entity).despawn_recursive();
    }
    let mut z = 0.;
    for node in graph_defn.graph.keys() {
      spawn_node(z, node, &mut commands, &asset_server);
      z += 1.;
    }
  } else {

  }
}

fn spawn_node(z: f32,
              node_name: &String,
              commands: &mut Commands,
              asset_server: &Res<AssetServer>,
) {
  //TODO move this to setup
  let font = asset_server.load("fonts/FiraSans-Bold.ttf");
  let text_style = TextStyle {
    font: font.clone(),
    font_size: 16.0,
    color: Color::WHITE
  };

  let shape = shapes::RegularPolygon {
    sides: 4,
    feature: shapes::RegularPolygonFeature::Radius(30.0),
    ..shapes::RegularPolygon::default()
  };

  let shape_outer = shapes::RegularPolygon {
    sides: 4,
    feature: shapes::RegularPolygonFeature::Radius(70.0),
    ..shapes::RegularPolygon::default()
  };
  let text_alignment = TextAlignment::Center;

  let mut rng = rand::thread_rng();
  let x = rng.gen_range(-250.0..250.0);
  let y = rng.gen_range(-250.0..250.0);

  let tween = Tween::new(
    EaseFunction::QuadraticInOut,
    Duration::from_millis(500),
    TransformPositionLens {
      start: Vec3::ZERO,
      end: Vec3::new(x, y, z),
    },
  );

  let parent = commands.spawn((
    SpatialBundle {
      transform: Transform::from_translation(Vec3::new(0., 0., 100.)),
      ..Default::default()
    },
    PickableBundle::default(),
    Node {node_text: node_name.to_string()},
    Animator::new(tween),
  )).id();

  let icon_child = commands.spawn((
    ShapeBundle {
      path: GeometryBuilder::build_as(&shape),
      ..default()
    },
    On::<Pointer<Drag>>::target_component_mut::<Parent>(|drag, p| {
      println!("got parent {}", p.index());

      // parent.transform.translation.x = transform.translation.x + drag.delta.x;
      // transform.translation.y = transform.translation.y - drag.delta.y;
    }),
    Fill::color(Color::YELLOW),
    Stroke::new(Color::BLACK, 3.0),
    PickableBundle::default(),
  )).id();

  let text_child = commands.spawn((Text2dBundle {
    text: Text::from_section(node_name, text_style).with_alignment(text_alignment),
    transform: Transform::from_translation(Vec3::new(0.0, -35., 100.)),
    ..default()
  }, PickableBundle::default())).id();

  commands.entity(parent).push_children(&[icon_child, text_child]);
}

pub fn update_connectors(graph_defn: Res<GraphDefinition>,
                         mut _code_store: ResMut<CodeStorage>,
                         mut commands: Commands,
                         window: Query<Entity, With<PrimaryWindow>>,
                         query: Query<(&Node, &Transform), Changed<Transform>>,
                         query_all: Query<(&Node, &Transform)>,
                         query_conn: Query<(Entity, &Connector)>
) {
  if !query.is_empty() {
    let mut all_node_loc = HashMap::<String, Vec3>::new();
    // let mut changed_node_loc = HashMap::<String, Vec3>::new();
    //despawn all connectors,
    //TODO we should despawn only required
    for (entity, _conn) in query_conn.iter() {
      commands.entity(entity).despawn_recursive();
    }

    //insert in hash map location of each node that has changed
    //when above TODO is done uncomment this
    // for (node, transform) in query.iter() {
    //   println!("changed: {}", node.node_text);
    //   let mut pos: Vec3 = transform.translation;
    //   pos.z = 50.;
    //   changed_node_loc.insert(node.node_text.clone(), pos);
    // }

    //insert in hash map location of all nodes
    for (node, transform) in query_all.iter() {
      let mut pos: Vec3 = transform.translation;
      pos.z = 50.;
      all_node_loc.insert(node.node_text.clone(), pos);
    }
    let mut done :HashSet<String> = HashSet::new();
    // for each node search its connecting entities
    // and make a line between current node and that node
    for (nodea_name, nodea_loc) in all_node_loc.iter() {
      for nodeb in graph_defn.graph.get(nodea_name).unwrap() {
        if !done.contains(&(nodea_name.clone() + nodeb)) {
          let nodeb_loc = all_node_loc.get(nodeb).unwrap();
          done.insert(nodea_name.clone() + nodeb);
          done.insert(nodeb.clone() + nodea_name);
          generate_line(nodea_loc, nodeb_loc, &window, &mut commands);
        }
      }
    }
  }
}

fn generate_line(a: &Vec3,
                 b: &Vec3,
                 window: &Query<Entity, With<PrimaryWindow>>,
                 commands: &mut Commands
) {
  let mut path_builder = PathBuilder::new();
  path_builder.move_to(Vec2{
    x: a.x, y: a.y
  });
  path_builder.cubic_bezier_to(
    Vec2::new(a.x + 50., a.y + 50.),
    Vec2::new(b.x + 50., b.y + 50.),
    Vec2::new(b.x, b.y),
  );
  let path = path_builder.build();
  commands.spawn((
    ShapeBundle {
      path,
      transform: Transform::from_xyz(0., 0., -50.),
      ..default()
    },
    Stroke::new(Color::BLACK, 3.0),
    Connector {
      _conn_text: "".to_string(),
    },
    On::<Pointer<Over>>::target_component_mut::<Stroke>(|_, s| {
      s.color = Color::BEIGE;
    }),
    On::<Pointer<Out>>::target_component_mut::<Stroke>(|_, s| {
      s.color = Color::BLACK;
    }),
  ));
}
