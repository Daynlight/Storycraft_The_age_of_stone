use bevy::prelude::*;

use crate::scenes;
use crate::config::settings;
use crate::mechanics::movement;




fn apply_movement(
  mut movers: Query<(&mut Transform, &movement::components::EntityVelocityVector)>
) {
  for (mut transform, velocity) in movers.iter_mut() {
    velocity.apply_to_transform(&mut transform, settings::FIXED_UPDATE_DELTA_TIME);
  }
}


fn movement_system_is_on(systems: Res<scenes::register::RunningSystemsRegister>) -> bool {
  systems.movement
}


pub struct MovementPlugin;
impl Plugin for MovementPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(FixedUpdate, apply_movement.run_if(movement_system_is_on));
  }
}
