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
  *systems = scenes::register::RunningSystemsRegister::default();
  for entity in &query {
    commands.entity(entity).despawn();
  }

  // set scene
  systems.movement = true;
  systems.player_events = true;
  systems.camera_tracking = true;

  prefabs::camera::MainCamera::spawn(&mut commands);
  prefabs::player::Player::spawn(&mut commands, &asset_server);

  // add Counter
  let texture = asset_server.load("Restaurant/Counter/Counter.png");
  commands.spawn((
    Sprite::from_image(texture), tags::GameEntity,
    Transform::from_xyz(0.0, 0.0, 0.0)
  ));
}


pub fn check(active: Res<scenes::system::ActiveScene>) -> bool {
  **active == scenes::register::ScenesRegister::Game
}