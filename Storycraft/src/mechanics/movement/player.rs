use bevy::prelude::*;

use crate::config::settings;
use crate::mechanics;
use crate::scenes;



fn set_player_velocity(
  player_movement_data: Res<mechanics::player_events::components::PlayerMovementData>,
  player: Single<(&mut mechanics::movement::components::VelocityVector, &mechanics::movement::components::MovementData)>
) {
  let (mut velocity_vector, movement_data) = player.into_inner();

  let target = player_movement_data.movement_direction * movement_data.max_velocity;
  let distance = target - velocity_vector.0;

  if distance.length() < movement_data.acceleration * settings::FIXED_UPDATE_DELTA_TIME {
    velocity_vector.0 = target;
  } else {
    let delta = distance.normalize_or_zero() * movement_data.acceleration * settings::FIXED_UPDATE_DELTA_TIME;
    velocity_vector.0 += delta;
  }
}


fn player_movement_system_is_on(systems: Res<scenes::register::RunningSystemsRegister>) -> bool {
  systems.player_movement
}


pub struct SetPlayerMovementPlugin;
impl Plugin for SetPlayerMovementPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(PreUpdate, set_player_velocity.run_if(player_movement_system_is_on));
  }
}
