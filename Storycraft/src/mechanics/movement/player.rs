use bevy::prelude::*;

use crate::config::settings;
use crate::mechanics;
use crate::scenes;



fn set_player_velocity(
  player: Single<(&mut mechanics::movement::components::EntityVelocityVector, &mechanics::movement::components::EntityMovementData)>
) {
  let (mut velocity_vector, entity_movement_data) = player.into_inner();

  let target = entity_movement_data.movement_direction * entity_movement_data.max_velocity;
  let distance = target - velocity_vector.0;

  if distance.length() < entity_movement_data.acceleration * settings::FIXED_UPDATE_DELTA_TIME {
    velocity_vector.0 = target;
  } else {
    let delta = distance.normalize_or_zero() * entity_movement_data.acceleration * settings::FIXED_UPDATE_DELTA_TIME;
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
