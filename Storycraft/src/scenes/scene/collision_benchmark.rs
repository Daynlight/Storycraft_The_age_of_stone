use bevy::prelude::*;

use crate::components::tags;
use crate::scenes;
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

  prefabs::camera::MainCamera::spawn(&mut commands);
  prefabs::player::Player::spawn(&mut commands, &asset_server);

  
  // add boxes
  for _ in 0..100{
    commands.spawn((
      Sprite{
        color: Color::linear_rgb(0.2, 0.8, 0.3),
        custom_size: Some(Vec2::new(32.0, 32.0)),
        ..default()
      }, tags::GameEntity,
      Transform::from_xyz(0.0, 0.0, 0.0)
    ));
  }
}


pub fn check(active: Res<scenes::system::ActiveScene>) -> bool {
  **active == scenes::register::ScenesRegister::CollisionBenchmark
}