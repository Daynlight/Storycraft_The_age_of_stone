use bevy::prelude::*;

use crate::mechanics;



pub struct MechanicsPlugin;
impl Plugin for MechanicsPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins(mechanics::player_events::systems::PlayerEventsPlugin)
    .add_plugins(mechanics::camera_tracking::systems::CameraTrackingPlugin)
    .add_plugins(mechanics::movement::systems::MovementPlugin)
    .add_plugins(mechanics::movement::player::SetPlayerMovementPlugin);
  }
}