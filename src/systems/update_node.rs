use bevy::prelude::*;
use crate::ui::*;
use crate::components::node::Node;
use crate::components::node::NodeTimers;
use bevy_prototype_lyon::prelude::*;
use bevy_tweening::{*, lens::*};
use rand::Rng;
use std::time::Duration;
use crate::systems::drag;
use bevy_mod_picking::prelude::*;

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
      spawn_node(z, node.clone(), &mut commands, &asset_server);
      z += 1.;
    }
  } else {

  }
}

fn spawn_node(z: f32,
              node_name: String,
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
    Node {node_text: node_name.clone(), ..Default::default()},
    Animator::new(tween),
  )).id();

  let icon_child = commands.spawn((
    ShapeBundle {
      path: GeometryBuilder::build_as(&shape),
      ..default()
    },

    On::<Pointer<DragStart>>::target_insert(Pickable::IGNORE),
    On::<Pointer<DragEnd>>::target_insert(Pickable::default()),
    On::<Pointer<Drag>>::run(drag::drag),
    On::<Pointer<Out>>::target_remove::<NodeTimers>(),
    Fill::color(Color::YELLOW),
    Stroke::new(Color::BLACK, 3.0),
  )).id();

  let text_child = commands.spawn((Text2dBundle {
    text: Text::from_section(node_name, text_style).with_alignment(text_alignment),
    transform: Transform::from_translation(Vec3::new(0.0, -35., 100.)),
    ..default()
  }, Pickable::IGNORE)).id();

  commands.entity(parent).push_children(&[icon_child, text_child]);
}


#[test]
fn did_spawn_node() {
  use std::collections::HashSet;
  use bevy::asset::AssetServer;
  use bevy::asset::FileAssetIo;
  use bevy::tasks::IoTaskPool;
  let mut app = App::new();
  let mut graph = NodeDepsMap::new();
  let mut hs = HashSet::new();
  hs.insert("b".to_string());
  graph.insert("a".to_string(), hs.clone());
  graph.insert("b".to_string(), hs);

  app.insert_resource(GraphDefinition {
    graph
  });
  IoTaskPool::init(Default::default);
  app.insert_resource(AssetServer::new(FileAssetIo::new("./assets", &None)));
  app.add_systems(Update, update_nodes);
  app.update();
  assert_eq!(app.world.query::<&Node>().iter(&app.world).count(), 2); // check all the keys have been spawned
  app.world.resource_mut::<GraphDefinition>().graph.clear();
  app.update();
  assert_eq!(app.world.query::<&Node>().iter(&app.world).count(), 0); // check if we change the graph the response is acceptable

}
