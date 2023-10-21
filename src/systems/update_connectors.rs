use bevy::prelude::*;
use crate::components::node_connector::*;
use crate::components::node::Node;
use std::collections::{HashMap, HashSet};
use bevy_prototype_lyon::prelude::*;
use bevy_mod_picking::prelude::*;
use crate::ui::CodeStorage;
use crate::ui::GraphDefinition;

pub fn update_connectors(graph_defn: Res<GraphDefinition>,
                         mut _code_store: ResMut<CodeStorage>,
                         mut commands: Commands,
                         query_changed: Query<(&Node, &Transform), Changed<Transform>>,
                         query_added: Query<(&Node, &Transform), Added<Transform>>,
                         query_all: Query<(&Node, &Transform)>,
                         mut query_conn: Query<(Entity, &mut Path, &mut NodeConnector)>
) {
  if !query_added.is_empty() {
    println!("added {} nodes", query_added.iter().count());
    let mut all_node_loc = HashMap::<String, Vec3>::new();

    for (entity, _path,  _conn) in query_conn.iter_mut() {
      commands.entity(entity).despawn_recursive();
    }

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
          let _ = generate_line(nodea_loc, nodeb_loc, &nodea_name, &nodeb, &mut commands);
        }
      }
    }
  } else if !query_changed.is_empty() {
    let mut all_node_loc = HashMap::<String, Vec3>::new();

    for (node, transform) in query_all.iter() {
      let mut pos: Vec3 = transform.translation;
      pos.z = 50.;
      all_node_loc.insert(node.node_text.clone(), pos);
    }

    for (_, mut path, mut conn) in query_conn.iter_mut() {
      let node1_loc = all_node_loc.get(&conn.node1);
      let node2_loc = all_node_loc.get(&conn.node2);

      let mut path_builder = PathBuilder::new();

      path_builder.move_to(Vec2{
        x: node1_loc.unwrap().x, y: node1_loc.unwrap().y
      });
      path_builder.cubic_bezier_to(
        Vec2::new(node1_loc.unwrap().x + 50., node1_loc.unwrap().y + 50.),
        Vec2::new(node2_loc.unwrap().x + 50., node2_loc.unwrap().y + 50.),
        Vec2::new(node2_loc.unwrap().x, node2_loc.unwrap().y),
      );

      *path = path_builder.build();
      (*conn).path = path.0.clone();
    }
  }
}


fn generate_line(a: &Vec3,
                 b: &Vec3,
                 node1: &String,
                 node2: &String,
                 commands: &mut Commands
) -> Entity {

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
  let walking_path = path.0.clone();
  commands.spawn((
    ShapeBundle {
      path,
      transform: Transform::from_xyz(0., 0., -50.),
      ..default()
    },
    Stroke::new(Color::BLACK, 3.0),
    NodeConnector {
      node1: node1.clone(),
      node2: node2.clone(),
      path: walking_path
    },
    On::<Pointer<Over>>::target_component_mut::<Stroke>(|_, s| {
      s.color = Color::BEIGE;
    }),
    On::<Pointer<Out>>::target_component_mut::<Stroke>(|_, s| {
      s.color = Color::BLACK;
    }),
  )).id()
}