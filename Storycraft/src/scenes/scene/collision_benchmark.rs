use bevy::prelude::*;
use rand::RngExt;

use crate::components::tags;
use crate::config::benchmark;
use crate::scenes;
use crate::prefabs;



pub fn set(
  mut commands: Commands,
  mut systems: ResMut<scenes::register::RunningSystemsRegister>,
  query: Query<Entity, With<tags::GameEntity>>,
) {
  // clear previous scene
  *systems = scenes::register::RunningSystemsRegister::default();
  for entity in &query {
    commands.entity(entity).despawn();
  }

  systems.movement = true;

  prefabs::camera::MainCamera::spawn(&mut commands);

  
  // add boxes
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


pub fn check(active: Res<scenes::system::ActiveScene>) -> bool {
  **active == scenes::register::ScenesRegister::CollisionBenchmark
}