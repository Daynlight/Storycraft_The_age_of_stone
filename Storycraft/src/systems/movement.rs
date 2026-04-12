use bevy::prelude::*;
use crate::player::{Player, PLAYER_VELOCITY};
use crate::camera::MainCamera;



pub fn camera_follow_player(
  camera: &mut Transform,
  player: &Transform,
) {
  camera.translation = player.translation;
}


pub fn player_movement(
  keyboard: Res<ButtonInput<KeyCode>>,
  time: Res<Time>,
  player_transform: &mut Transform
) {
  let speed = PLAYER_VELOCITY * time.delta_secs();

  if keyboard.pressed(KeyCode::KeyW) {
    player_transform.translation.y += speed;
  }
  if keyboard.pressed(KeyCode::KeyS) {
    player_transform.translation.y -= speed;
  }
  if keyboard.pressed(KeyCode::KeyA) {
    player_transform.translation.x -= speed;
  }
  if keyboard.pressed(KeyCode::KeyD) {
    player_transform.translation.x += speed;
  }
}


pub fn movement_system(
  keyboard: Res<ButtonInput<KeyCode>>,
  time: Res<Time>,
  camera: Single<&mut Transform, (With<MainCamera>, Without<Player>)>,
  player: Single<&mut Transform, With<Player>>,
) {
  let mut player_transform = player.into_inner();
  let mut camera_transform = camera.into_inner();

  player_movement(keyboard, time, &mut player_transform);
  camera_follow_player(&mut camera_transform, &player_transform);
}
