use bevy::prelude::*;



#[derive(Component)]
pub struct Movement{
  current_position: Vec2,
  direction: Vec2,
  max_velocity: f32,
  current_velocity: f32,
  acceleration: f32,
  resistance: f32,
}


impl Movement{
  pub fn new(position: Vec2, max_velocity: f32, acceleration: f32, resistance: f32) -> Self {
    Movement {
      current_position: position,
      direction: Vec2::ZERO,
      max_velocity: max_velocity,
      current_velocity: 0.0,
      acceleration: acceleration,
      resistance: resistance
    }
  }


  pub fn get_current_position(&self) -> Vec2 {
    return self.current_position;
  }


  pub fn update_current_position(&mut self, delta_time: f32) {
    self.current_position += self.direction * self.current_velocity * delta_time;
  }


  pub fn set_movement(&mut self, direction: Vec2, delta_time: f32) {
    // [TODO] Energy loss at direction change relative to angle
    // [TODO] Fraction of direction angle

    // resistance
    if self.current_velocity < 0.0{
      self.current_velocity = 0.0;
      self.direction = Vec2::ZERO;
      return;
    }
    if direction == Vec2::ZERO {
      self.current_velocity -= self.resistance * delta_time;
    }
    
    // acceleration
    if self.current_velocity > self.max_velocity {
      self.current_velocity = self.max_velocity;
    }
    if direction != Vec2::ZERO {
      self.current_velocity += self.acceleration * delta_time;
      self.direction = direction;
    }
  }
}
