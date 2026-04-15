use bevy::prelude::*;

use crate::components::movement::Movement;
use crate::prefabs::camera::MainCamera;
use crate::prefabs::player::Player;
use crate::config;
use crate::components;



#[derive(Component, Default)]
pub struct PlayerInput {
  pub move_dir: Vec2
}

pub fn update_camera_to_follow_player(
  camera: &mut Transform,
  player: &Transform,
) {
  camera.translation = player.translation;
}


pub fn get_direction_base_on_controls(
  keyboard: Res<ButtonInput<KeyCode>>,
) -> Vec2 {
  let mut direction = Vec2::ZERO;

  if keyboard.any_pressed(config::controls::UP) {
    direction.y = 1.0;
  }
  if keyboard.any_pressed(config::controls::DOWN) {
    direction.y = -1.0;
  }
  if keyboard.any_pressed(config::controls::LEFT) {
    direction.x = -1.0;
  }
  if keyboard.any_pressed(config::controls::RIGHT) {
    direction.x = 1.0;
  }

  return direction;
}


pub fn player_movement(
  keyboard: Res<ButtonInput<KeyCode>>,
  time: Res<Time>,
  player_input: &mut PlayerInput,
  player_movement: &mut components::movement::Movement,
  player_transform: &mut Transform,
) {
  let direction = get_direction_base_on_controls(keyboard).normalize_or_zero();
  player_input.move_dir = direction;

  // player_movement.make_move(direction, time.delta_secs());

  let current_position = player_movement.get_current_position();
  // player_transform.translation = Vec3::new(current_position.x, current_position.y, player_transform.translation.z);
}


pub fn movement_system(
  keyboard: Res<ButtonInput<KeyCode>>,
  time: Res<Time>,
  camera: Single<&mut Transform, (With<MainCamera>, Without<Player>)>,
  player: Single<(&mut Movement, &mut Transform, &mut PlayerInput), With<Player>>,
) {
  let mut player_transform = player.into_inner();
  let mut camera_transform = camera.into_inner();

  player_movement(keyboard, time, &mut player_transform.2, &mut player_transform.0, &mut player_transform.1 );
  update_camera_to_follow_player(&mut camera_transform, &player_transform.1);
}
