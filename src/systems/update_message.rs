use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::message::*;
use crate::components::node_connector::*;

fn walk_message(path: &lyon_algorithms::path::Path) -> Vec<[f32; 2]> {
  use lyon_algorithms::walk::{RegularPattern, walk_along_path, WalkerEvent};

  let mut x: Vec<[f32; 2]> = vec!();
  let mut pattern = RegularPattern {
    callback: &mut |event: WalkerEvent| {
      x.push(event.position.to_array());
      true // Return true to continue walking the path.
    },
    // Invoke the callback above at a regular interval of 3 units.
    interval: 0.1,
  };


  let tolerance = 0.1; // The path flattening tolerance.
  let start_offset = 0.0; // Start walking at the beginning of the path.

  walk_along_path(
    path.iter(),
    start_offset,
    tolerance,
    &mut pattern
  );
  x
}

pub fn update_message_path(mut query_conn: Query<(Entity, &mut Message, &NodeConnector)>,
                           mut query_hs: Query<(Entity, &Parent, &HotSpot)>,
                           time: Res<Time>,
                           mut commands: Commands,
) {
  for (entity, parent, _hotspot) in query_hs.iter_mut() {
    commands.entity(parent.get()).remove_children(&[entity]);
    commands.entity(entity).despawn_recursive();
  }

  for (entity, mut mesg, nc) in query_conn.iter_mut() {
    mesg.timer.tick(time.delta());
    let v_points = walk_message(&nc.path);
    let perc_elap : f64 = mesg.timer.elapsed().as_millis() as f64 / mesg.timer.duration().as_millis() as f64;
    let loc = perc_elap * v_points.len() as f64;
    let loc_int = loc.round() as usize;
    if loc_int == v_points.len() as usize {
      //remove message
    } else {

      let shape = shapes::RegularPolygon {
        sides: 4,
        feature: shapes::RegularPolygonFeature::Radius(4.0),
        ..shapes::RegularPolygon::default()
      };
      let ch_en = commands.spawn((
        HotSpot{},
        ShapeBundle {
          path: GeometryBuilder::build_as(&shape),
          transform: Transform::from_xyz(v_points[loc_int][0], v_points[loc_int][1], -10.0),
          ..default()
        },
        Stroke::new(Color::BLACK, 3.0),
      )).id();
      commands.entity(entity).add_child(ch_en);
    }
  }
}
