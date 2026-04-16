use bevy::prelude::*;



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
  pub fn apply_to_transform(&self, transform: &mut Transform, delta_time: f32) {
    transform.translation.x += self.x * delta_time;
    transform.translation.y += self.y * delta_time;
  }
}