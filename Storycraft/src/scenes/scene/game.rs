use bevy::prelude::*;

use crate::scenes;
use crate::tags;
use crate::prefabs;



pub fn set(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut systems: ResMut<scenes::register::RunningSystemsRegister>,
  query: Query<Entity, With<tags::GameEntity>>,
) {
  // clear previous scene
  for entity in &query {
    commands.entity(entity).despawn();
  }

  // set systems
  *systems = scenes::register::RunningSystemsRegister{
    movement: true,
    player_events: true,
    camera_tracking: true,
    player_movement: true,
  };

  // compose scene
  prefabs::game_camera::GameCamera::spawn(&mut commands);
  prefabs::player::Player::spawn(&mut commands, &asset_server);

  //// add Counter
  let texture = asset_server.load("Restaurant/Counter/Counter.png");
  commands.spawn((
    Sprite::from_image(texture), tags::GameEntity,
    Transform::from_xyz(0.0, 0.0, 0.0)
  ));
}


pub fn check(active: Res<scenes::system::ActiveScene>) -> bool {
  **active == scenes::register::ScenesRegister::Game
}
