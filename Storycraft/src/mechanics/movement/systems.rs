use bevy::prelude::*;

use crate::scenes;
use crate::config::settings;
use crate::mechanics::movement;
use crate::mechanics::player_events;




fn apply_movement(
  mut movers: Query<(&mut Transform, &movement::components::VelocityVector)>
) {
  for (mut transform, velocity) in movers.iter_mut() {
    velocity.apply_to_transform(&mut transform, settings::FIXED_UPDATE_DELTA_TIME);
  }
}


fn set_player_velocity(
  player_movement_data: Res<player_events::components::PlayerMovementData>,
  player: Single<(&mut movement::components::VelocityVector, &movement::components::MovementData)>
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


fn movement_system_is_on(systems: Res<scenes::register::RunningSystemsRegister>) -> bool {
  systems.movement
}


pub struct MovementPlugin;
impl Plugin for MovementPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(FixedUpdate, (set_player_velocity, apply_movement).chain().run_if(movement_system_is_on));
  }
}
