use bevy::prelude::*;

use crate::utils::tags;
use crate::config;
use crate::scenes;
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
    player_movement: false,
    collisions: true,
  };

  // compose scene
  prefabs::game_camera::GameCamera::spawn(&mut commands, config::CAMERA_ZOOM);
  prefabs::player::Player::spawn(&mut commands, &asset_server, Vec3::new(1.0, 1.0, 0.0));

  // // Tiles
  prefabs::dungeon::DungeonTile::from_range(&mut commands, &asset_server, IVec3::new(-10, -10, 0), IVec3::new(10, 10, 0));

  // // Walls
  prefabs::dungeon::DungeonWall::from_range(&mut commands, &asset_server, IVec3::new(11, -10, 0), IVec3::new(11, 10, 1));
  prefabs::dungeon::DungeonWall::from_range(&mut commands, &asset_server, IVec3::new(-10, 11, 0), IVec3::new(10, 11, 1));

  prefabs::dungeon::DungeonWall::from_range(&mut commands, &asset_server, IVec3::new(0, 5, 0), IVec3::new(10, 5, 1));

  prefabs::collider::Collider::from_range(&mut commands, IVec3::new(-11, -11, 0), IVec3::new(11, -11, 0));
  prefabs::collider::Collider::from_range(&mut commands, IVec3::new(-11, -10, 0), IVec3::new(-11, 11, 0));

  // Lamp
  prefabs::dungeon::DungeonStandingLamp::spawn(&mut commands, &asset_server, Vec3::new(-5.0, -5.0, 0.0));

}


pub fn check(active: Res<scenes::plugins::ActiveScene>) -> bool {
  **active == scenes::register::ScenesRegister::Game
}
