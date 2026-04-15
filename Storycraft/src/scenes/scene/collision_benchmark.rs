use bevy::prelude::*;
use rand::RngExt;

use crate::components::tags;
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

  prefabs::camera::MainCamera::spawn(&mut commands);

  
  // add boxes
  for _ in 0..1000{
    let mut rng = rand::rng();
    let x: f32 = rng.random_range(-300..300) as f32;
    let y: f32 = rng.random_range(-200..200) as f32;

    commands.spawn((
      Sprite{
        color: Color::linear_rgb(0.0, 0.5, 0.5),
        custom_size: Some(Vec2::new(10.0, 10.0)),
        ..default()
      }, tags::GameEntity,
      Transform::from_xyz(x, y, 0.0)
    ));
  }
}


pub fn check(active: Res<scenes::system::ActiveScene>) -> bool {
  **active == scenes::register::ScenesRegister::CollisionBenchmark
}