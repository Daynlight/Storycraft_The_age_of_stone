use bevy::prelude::*;
use rand::RngExt;

use crate::utils::tags;
use crate::config;
use crate::scenes;
use crate::prefabs;



pub fn set(
  mut commands: Commands,
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
    camera_tracking: false,
    player_movement: false,
    collisions: true,
  };

  // compose scene 
  prefabs::game_camera::GameCamera::spawn(&mut commands, 1.0);

  // edge boxes
  let position = config::COLLISION_BOX_EDGE;
  let edge_size = config::COLLISION_BOX_EDGE_SIZE;
  
  prefabs::benchmark_collsion::StaticCollisionBox::spawn(&mut commands, Vec2::new(-edge_size, position.y + edge_size / 2.0), Vec2::new(2.0 * (position.x + 2.0 * edge_size), edge_size));
  prefabs::benchmark_collsion::StaticCollisionBox::spawn(&mut commands, Vec2::new(-edge_size, -position.y - edge_size / 2.0), Vec2::new(2.0 * (position.x + 2.0 * edge_size), edge_size));
  prefabs::benchmark_collsion::StaticCollisionBox::spawn(&mut commands, Vec2::new(position.x + edge_size / 2.0, 0.0), Vec2::new(edge_size, 2.0 * position.y));
  prefabs::benchmark_collsion::StaticCollisionBox::spawn(&mut commands, Vec2::new(-position.x - edge_size / 2.0, 0.0), Vec2::new(edge_size, 2.0 * position.y));
  
  //// add boxes
  let generate_position = position - config::COLLISION_BOX_MARGIN;
  let generate_direction = Vec2::new(100.0, 100.0);

  for _ in 0..config::COLLISION_BOX_BENCHMARK_AMOUNT{
    let mut rng = rand::rng();
    let x: f32 = rng.random_range(-generate_position.x..generate_position.x) as f32;
    let y: f32 = rng.random_range(-generate_position.y..generate_position.y) as f32;
    let position = Vec2::new(x, y);

    let x: f32 = rng.random_range(-generate_direction.x..generate_direction.x) as f32;
    let y: f32 = rng.random_range(-generate_direction.y..generate_direction.y) as f32;

    let velocity: f32 = rng.random_range(0.0..config::COLLISION_BOX_MAX_VELOCITY) as f32;
    let direction = Vec2::new(x, y).normalize_or_zero() * velocity;

    prefabs::benchmark_collsion::CollisionBox::spawn(&mut commands, position, direction, Vec2::new(5.0, 5.0));
  }

  for _ in 0..config::COLLISION_BOX_STATIC_BENCHMARK_AMOUNT{
    let mut rng = rand::rng();
    let x: f32 = rng.random_range(-generate_position.x..generate_position.x) as f32;
    let y: f32 = rng.random_range(-generate_position.y..generate_position.y) as f32;
    let position = Vec2::new(x, y);

    prefabs::benchmark_collsion::StaticCollisionBox::spawn(&mut commands, position, Vec2::new(5.0, 5.0));
  }
}


pub fn check(active: Res<scenes::plugins::ActiveScene>) -> bool {
  **active == scenes::register::ScenesRegister::CollisionBenchmark
}
