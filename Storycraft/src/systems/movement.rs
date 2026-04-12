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
  let speed = PLAYER_VELOCITY * time.delta_seconds();

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
  mut camera: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
  mut player: Query<&mut Transform, With<Player>>,
) {
  if let Ok(mut player_transform) = player.get_single_mut() {
    if let Ok(mut camera_transform) = camera.get_single_mut(){
      player_movement(keyboard, time, &mut player_transform);
      camera_follow_player(&mut camera_transform, &player_transform);
    }
  }
}
