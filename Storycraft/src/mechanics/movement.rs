use bevy::prelude::*;

use crate::scenes;
use crate::utils::utils;
use crate::config;
use crate::mechanics::movement;



#[derive(Component, Default, Deref, DerefMut)]
pub struct WorldPos(pub Vec3);


#[derive(Component, Default)]
pub struct EntityMovementData {
  pub movement_direction: Vec2,
  pub max_velocity: f32,
  pub acceleration: f32
}

impl EntityMovementData {
  pub fn new(max_velocity: f32, acceleration: f32) -> Self {
    EntityMovementData {
      max_velocity,
      acceleration,
      ..default()
    }
  }
}



#[derive(Component, Deref, DerefMut, Default)]
pub struct EntityVelocityVector(pub Vec2);

impl EntityVelocityVector {
  pub fn apply_to_transform(&self, world_pos: &mut WorldPos, transform: &mut Transform, delta_time: f32) {
    world_pos.x += self.x * delta_time;
    world_pos.y += self.y * delta_time;
    
    let position = utils::world_to_view(world_pos.0);
    transform.translation = position;
  }
}


fn apply_movement(
  mut movers: Query<(&mut WorldPos, &mut Transform, &movement::EntityVelocityVector)>
) {
  for (mut world_pos, mut transform, velocity) in movers.iter_mut() {
    velocity.apply_to_transform(&mut world_pos, &mut transform, config::FIXED_UPDATE_DELTA_TIME);
  }
}



pub struct MovementPlugin;
impl Plugin for MovementPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(FixedUpdate, apply_movement
      .run_if(|systems: Res<scenes::register::RunningSystemsRegister>| { systems.movement })
    );
  }
}
