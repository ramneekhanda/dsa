use crate::resources::graph_def::NodeTicked;
use bevy::prelude::*;

/// Pulse zoom animation disabled per user request.
pub fn pulse_on_tick(mut ticked_reader: EventReader<NodeTicked>) {
    ticked_reader.clear();
}
