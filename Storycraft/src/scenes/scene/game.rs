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
    camera_tracking: true,
    player_movement: true,
    collisions: true,
  };

  // compose scene
  prefabs::game_camera::GameCamera::spawn(&mut commands);
  prefabs::player::Player::spawn(&mut commands, &asset_server);

  //// add Counter
  prefabs::counter::Counter::spawn(&mut commands, &asset_server, Vec2::new(0.0, 0.0), Vec2::new(32.0, 32.0));
}


pub fn check(active: Res<scenes::plugins::ActiveScene>) -> bool {
  **active == scenes::register::ScenesRegister::Game
}
