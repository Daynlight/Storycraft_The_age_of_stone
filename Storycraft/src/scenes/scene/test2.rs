use bevy::prelude::*;

use crate::components::tags;
use crate::scenes;
use crate::prefabs;



pub fn set(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut systems: ResMut<scenes::register::SceneSystemsRegister>,
  query: Query<Entity, With<tags::GameEntity>>,
) {
  // clear previous scene
  *systems = scenes::register::SceneSystemsRegister::default();
  for entity in &query {
    commands.entity(entity).despawn();
  }

  // set scene
  systems.movement = true;

  prefabs::camera::MainCamera::spawn(&mut commands);
  prefabs::player::Player::spawn(&mut commands, &asset_server);

  // add Counter
  let texture = asset_server.load("Restaurant/Counter/Counter.png");
  commands.spawn((
    Sprite::from_image(texture), tags::GameEntity,
    Transform::from_xyz(0.0, 0.0, 0.0)
  ));

  // add Counter
  let texture = asset_server.load("Restaurant/Counter/Counter.png");
  commands.spawn((
    Sprite::from_image(texture), tags::GameEntity,
    Transform::from_xyz(100.0, 0.0, 0.0)
  ));

  // add Counter
  let texture = asset_server.load("Restaurant/Counter/Counter.png");
  commands.spawn((
    Sprite::from_image(texture), tags::GameEntity,
    Transform::from_xyz(0.0, 100.0, 0.0)
  ));

  // add Counter
  let texture = asset_server.load("Restaurant/Counter/Counter.png");
  commands.spawn((
    Sprite::from_image(texture), tags::GameEntity,
    Transform::from_xyz(100.0, 100.0, 0.0)
  ));
}


pub fn in_test2_scene(active: Res<scenes::system::ActiveScene>) -> bool {
  **active == scenes::register::RegisteredScenes::Test2
}