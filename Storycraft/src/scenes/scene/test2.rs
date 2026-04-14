use bevy::prelude::*;

use crate::components::tags;
use crate::scenes;
use crate::prefabs;



pub fn set(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut systems: ResMut<scenes::register::SceneSystemsRegister>,
  query: Query<Entity, With<tags::GameEntity>>,
  active_scene: Res<scenes::system::ActiveScene>,
  mut last_scene: ResMut<scenes::system::LastScene>,
) {
  // validate event
  if **active_scene != scenes::register::RegisteredScenes::Test2 {
    return;
  }
  if **active_scene == **last_scene {
    return;
  }
  last_scene.0 = active_scene.0; 

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
