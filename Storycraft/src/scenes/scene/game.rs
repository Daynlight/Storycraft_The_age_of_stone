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
    player_movement: true,
    collisions: true,
  };

  // compose scene
  prefabs::game_camera::GameCamera::spawn(&mut commands, config::CAMERA_ZOOM);
  prefabs::player::Player::spawn(&mut commands, &asset_server, Vec3::new(1.0, 1.0, 1.0));


  let map_size = IVec2::new(-10, 10);
  // Tiles
  for x in map_size.x..map_size.y{
    for y in map_size.x..map_size.y{
      let position = Vec3::new(x as f32, y as f32, 0.0);
      prefabs::dungeon::DungeonTile::spawn(&mut commands, &asset_server, position);
    }
  }

  // Walls
  for x in map_size.x..map_size.y + 1{
    let position = Vec3::new(x as f32, -map_size.x as f32, 0.0); 
    prefabs::dungeon::DungeonWall::spawn(&mut commands, &asset_server, position);

    let position = Vec3::new(x as f32, -map_size.x as f32, 1.0); 
    prefabs::dungeon::DungeonWall::spawn(&mut commands, &asset_server, position);
  }
  for y in map_size.x..map_size.y{
    let position = Vec3::new(-map_size.x as f32, y as f32, 0.0); 
    prefabs::dungeon::DungeonWall::spawn(&mut commands, &asset_server, position);

    let position = Vec3::new(-map_size.x as f32, y as f32, 1.0); 
    prefabs::dungeon::DungeonWall::spawn(&mut commands, &asset_server, position);
  }
  for y in map_size.x..map_size.y + 1{
    let position = Vec3::new(map_size.x as f32, y as f32, 0.0); 
    prefabs::collider::Collider::spawn(&mut commands, position, Vec2::new(32.0, 32.0));
  }
  for x in map_size.x..map_size.y{
    let position = Vec3::new(x as f32, map_size.x as f32, 0.0); 
    prefabs::collider::Collider::spawn(&mut commands, position, Vec2::new(32.0, 32.0));
  }

  prefabs::dungeon::DungeonStandingLamp::spawn(&mut commands, &asset_server, Vec3::new(-5.0, -5.0, 1.0));

}


pub fn check(active: Res<scenes::plugins::ActiveScene>) -> bool {
  **active == scenes::register::ScenesRegister::Game
}
