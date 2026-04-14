use bevy::prelude::*;

use crate::prefabs;
use crate::scenes::scenes::SceneSystems;
use crate::components::tags::GameEntity;



pub fn set(
  mut commands: Commands,
  mut systems: ResMut<SceneSystems>,
  asset_server: Res<AssetServer>,
) {
  systems.movement = true;

  prefabs::camera::MainCamera::spawn(&mut commands);
  prefabs::player::Player::spawn(&mut commands, &asset_server);

  // add Counter
  let texture = asset_server.load("Restaurant/Counter/Counter.png");
  commands.spawn((
    Sprite::from_image(texture), GameEntity,
    Transform::from_xyz(64.0, 0.0, 0.0)
  ));

    // add Counter
  let texture = asset_server.load("Restaurant/Counter/Counter.png");
  commands.spawn((
    Sprite::from_image(texture), GameEntity,
    Transform::from_xyz(0.0, 45.0, 0.0)
  ));

    // add Counter
  let texture = asset_server.load("Restaurant/Counter/Counter.png");
  commands.spawn((
    Sprite::from_image(texture), GameEntity,
    Transform::from_xyz(21.0, 0.0, 0.0)
  ));

  // add Counter
  let texture = asset_server.load("Restaurant/Counter/Counter.png");
  commands.spawn((
    Sprite::from_image(texture), GameEntity,
    Transform::from_xyz(31.0, 64.0, 0.0)
  ));
}
