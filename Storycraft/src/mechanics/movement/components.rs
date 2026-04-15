use bevy::prelude::*;



#[derive(Component, Default)]
pub struct MovementData {
  pub max_velocity: f32,
  pub acceleration: f32,
  pub resistance: f32
}


impl MovementData {
  pub fn new(max_velocity: f32, acceleration: f32, resistance: f32) -> Self {
    MovementData {
      max_velocity,
      acceleration,
      resistance
    }
  }
}


#[derive(Component, Deref, DerefMut, Default)]
pub struct VelocityVector(pub Vec2);


impl VelocityVector {
  pub fn apply_to_transform(&self, transform: &mut Transform, delta_time: f32) {
    transform.translation.x += self.x * delta_time;
    transform.translation.y += self.y * delta_time;
  }
}