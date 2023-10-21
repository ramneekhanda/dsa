use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Message {
  pub timer: Timer,
  pub str: String,
  pub nodeFrom: String,
  pub nodeTo: String,
}

#[derive(Component, Debug, Clone)]
pub struct HotSpot {}
