use bevy::prelude::*;

use crate::player::{Player};








#[derive(Component)]
pub struct MainCamera;
const CAMERA_ZOOM: f32 = 0.5;


impl MainCamera{
  pub fn spawn(
    commands: &mut Commands,
  ){
    let mut camera: Camera2dBundle = Camera2dBundle::default();
    camera.projection.scale = CAMERA_ZOOM;

    commands.spawn((camera, MainCamera));
  }
}



pub fn camera_move_system(
  mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
  player_query: Query<&Transform, With<Player>>,
) {
  if let Ok(player_transform) = player_query.get_single() {
    let player_position = player_transform.translation;

    for mut transform in &mut camera_query {
      transform.translation = player_position;
    }
  }
}
