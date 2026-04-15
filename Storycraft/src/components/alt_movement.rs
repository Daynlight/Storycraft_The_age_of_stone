use bevy::prelude::*;

use crate::scenes;
use crate::systems::movement::PlayerInput;
use crate::components::movement;



#[derive(Component)]
pub struct MovementData {
  max_velocity: f32,
  acceleration: f32
}
impl MovementData {
  pub fn new(max_velocity: f32, acceleration: f32) -> Self {
    Self {
      max_velocity,
      acceleration
    }
  }
}

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);
impl Velocity {
  fn apply_to_transform(&self, transform: &mut Transform, delta_time: f32) {
    transform.translation.x += self.x * delta_time;
    transform.translation.y += self.y * delta_time;
  }
}

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(FixedUpdate, (set_player_velocity, apply_movement).chain().run_if(movement::movement_system_is_on));
  }
}



const DELTA_TIME: f32 = 1./64.;

fn apply_movement(
  mut movers: Query<(&mut Transform, &Velocity)>
) {
  for (mut transform, velocity) in movers.iter_mut() {
    velocity.apply_to_transform(&mut transform, DELTA_TIME);
  }
}


fn set_player_velocity(
  mut player: Single<(&PlayerInput, &MovementData, &mut Velocity)>
) {
  let (player_input, settings, mut velocity) = player.into_inner();

  let target = player_input.move_dir * settings.max_velocity;
  let distance = target - velocity.0;

  if distance.length() < settings.acceleration * DELTA_TIME {
    velocity.0 = target;
  } else {
    let delta = distance.normalize_or_zero() * settings.acceleration * DELTA_TIME;
    velocity.0 += delta;
  }
}
