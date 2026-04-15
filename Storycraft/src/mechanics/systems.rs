use bevy::prelude::*;

use crate::mechanics::player_events;
use crate::mechanics::camera_tracking;
use crate::mechanics::movement;


pub struct MechanicsPlugin;
impl Plugin for MechanicsPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins(player_events::systems::PlayerEventsPlugin)
    .add_plugins(camera_tracking::systems::CameraTrackingPlugin)
    .add_plugins(movement::systems::MovementPlugin);
  }
}