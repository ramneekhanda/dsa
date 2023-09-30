use crate::ui::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle, render::{
        mesh::{MeshVertexBufferLayout, PrimitiveTopology},
        render_resource::{
            AsBindGroup, PolygonMode, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError,
        },
    },};
use bevy_tweening::{*, lens::*};
use rand::Rng;
use std::collections::HashMap;
use rustpython_vm::Interpreter as py_interpreter;
use std::time::*;

type NodeDepsMap = HashMap::<String, Vec<String>>;
#[derive(Component, Debug)]
pub struct Node {
  node_text: String,
}

#[derive(Component, Debug)]
pub struct Connector {
  conn_text: String,
}

impl Default for Node {
    fn default() -> Self {
        Node {
          node_text: "ANODE".to_string()
        }
    }
}

#[derive(Bundle, Debug, Default)]
pub struct NodeBundle {
  /// The visibility of the entity.
  pub visibility: Visibility,
  /// The computed visibility of the entity.
  pub computed: ComputedVisibility,
  /// The transform of the entity.
  pub transform: Transform,
  /// The global transform of the entity.
  pub global_transform: GlobalTransform,

  pub node: Node,
}

pub fn update_nodes(
  mut commands: Commands,
  mut gizmos: Gizmos,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  asset_server: Res<AssetServer>,
  graph_defn: Res<GraphDefinition>,
  query: Query<Entity, With<Node>>,
) {
  if graph_defn.is_changed() {
    for entity in query.iter() {
      commands.entity(entity).despawn_recursive();
    }
    for node in graph_defn.graph.keys() {
      spawn_node(node, &mut commands, &asset_server, &mut meshes, &mut materials);
    }
  } else {

  }
}

fn spawn_node(node_name: &String,
              commands: &mut Commands,
              asset_server: &Res<AssetServer>,
              meshes: &mut ResMut<Assets<Mesh>>,
              materials: &mut ResMut<Assets<ColorMaterial>>,
) {
  let font = asset_server.load("fonts/FiraSans-Bold.ttf");
  let text_style = TextStyle {
    font: font.clone(),
    font_size: 16.0,
    color: Color::WHITE
  };
  let text_alignment = TextAlignment::Center;

  let mut rng = rand::thread_rng();
  let x = rng.gen_range(-250.0..250.0);
  let y = rng.gen_range(-250.0..250.0);

  let tween = Tween::new(
    // Use a quadratic easing on both endpoints.
    EaseFunction::QuadraticInOut,
    // Animation time (one way only; for ping-pong it takes 2 seconds
    // to come back to start).
    Duration::from_secs(2),
    // The lens gives the Animator access to the Transform component,
    // to animate it. It also contains the start and end values associated
    // with the animation ratios 0. and 1.
    TransformPositionLens {
      start: Vec3::ZERO,
      end: Vec3::new(x, y, 0.),
    },
  );


  let parent = commands.spawn((
    SpatialBundle {
      transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
      ..Default::default()
    },
    Node {node_text: node_name.to_string()},
    Animator::new(tween),
  )).id();
  let icon_child = commands.spawn(
    MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(25.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    }).id();

  let text_child = commands.spawn(Text2dBundle {
    text: Text::from_section(node_name, text_style).with_alignment(text_alignment),
    transform: Transform::from_translation(Vec3::new(0.0, -35., 0.)),
    ..default()
  }).id();

  commands.entity(parent).push_children(&[icon_child, text_child]);
}


pub fn update_connectors(graph_defn: Res<GraphDefinition>,
                         mut _code_store: ResMut<CodeStorage>,
                         mut commands: Commands,
                         mut meshes: ResMut<Assets<Mesh>>,
                         mut materials: ResMut<Assets<ColorMaterial>>,
                         query: Query<(&Node, &Transform), (Changed<Transform>)>,
                         query_conn: Query<(Entity, &Connector)>
) {
  if !query.is_empty() {
    let mut node_loc = HashMap::<String, Vec3>::new();
    for (entity, _conn) in query_conn.iter() {
      commands.entity(entity).despawn_recursive();
    }
    for (node, transform) in query.iter() {

      let pos: Vec3 = transform.translation;
      node_loc.insert(node.node_text.clone(), pos);
    }
    for nodea in graph_defn.graph.keys() {
      if node_loc.get(nodea) == None {
        //TODO???
      } else {
        let nodea_loc = node_loc.get(nodea).unwrap();

        for nodeb in graph_defn.graph.get(nodea).unwrap() {
          let nodeb_loc = node_loc.get(nodeb).unwrap();
          commands.spawn((
            MaterialMesh2dBundle {
              mesh: meshes.add(Mesh::from(LineStrip {
                points: vec![
                  *nodea_loc,
                  *nodeb_loc,
                ]
              })).into(),
              material: materials.add(ColorMaterial::from(Color::GREEN)),
              ..Default::default()
            }, Connector {
              conn_text: "".to_string(),
            }
          ));
        }
      }
    }
  }
}

#[derive(Debug, Clone)]
pub struct LineStrip {
    pub points: Vec<Vec3>,
}

impl From<LineStrip> for Mesh {
    fn from(line: LineStrip) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, line.points);
        mesh
    }
}
