use bevy::prelude::*;

use crate::prefabs;
use crate::config;
use crate::components;



pub fn update_camera_to_follow_player(
  camera: &mut Transform,
  player: &components::movement::Movement,
) {
  let position: Vec2 = player.get_current_position();
  camera.translation = Vec3::new(position.x, position.y, camera.translation.z);
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
  player_movement: &mut components::movement::Movement,
  player_transform: &mut Transform,
) {
  let direction = get_direction_base_on_controls(keyboard);

  player_movement.make_move(direction, time.delta_secs());

  let current_position = player_movement.get_current_position();
  player_transform.translation = Vec3::new(current_position.x, current_position.y, player_transform.translation.z);
}


pub fn movement_system(
  keyboard: Res<ButtonInput<KeyCode>>,
  time: Res<Time>,
  camera: Single<&mut Transform, (With<prefabs::camera::MainCamera>, Without<prefabs::player::Player>)>,
  player: Single<(&mut components::movement::Movement, &mut Transform), With<prefabs::player::Player>>,
) {
  let mut player_transform = player.into_inner();
  let mut camera_transform = camera.into_inner();

  player_movement(keyboard, time, &mut player_transform.0, &mut player_transform.1 );
  update_camera_to_follow_player(&mut camera_transform, &player_transform.0);
}
