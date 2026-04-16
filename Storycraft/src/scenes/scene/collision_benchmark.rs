use bevy::prelude::*;
use rand::RngExt;

use crate::tags;
use crate::config::benchmark;
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
    player_events: false,
    camera_tracking: false,
    player_movement: false,
  };

  // compose scene 
  prefabs::game_camera::GameCamera::spawn(&mut commands);

  //// add boxes
  for _ in 0..benchmark::COLLISIONBOXBENCHMARKAMMOUNT{
    let mut rng = rand::rng();
    let x: f32 = rng.random_range(-300..300) as f32;
    let y: f32 = rng.random_range(-200..200) as f32;
    let position = Vec2::new(x, y);

    let x: f32 = rng.random_range(-benchmark::COLLISIONBOXMAXVELOCITY..benchmark::COLLISIONBOXMAXVELOCITY) as f32;
    let y: f32 = rng.random_range(-benchmark::COLLISIONBOXMAXVELOCITY..benchmark::COLLISIONBOXMAXVELOCITY) as f32;
    let direction = Vec2::new(x, y);

    prefabs::collision_box::CollisionBox::spawn(&mut commands, position, direction);
  }
}


pub fn check(active: Res<scenes::systems::ActiveScene>) -> bool {
  **active == scenes::register::ScenesRegister::CollisionBenchmark
}
