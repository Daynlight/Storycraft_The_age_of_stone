use bevy::prelude::*; 

use crate::utils::tags;



#[derive(Component)]
pub struct GameCamera;


impl GameCamera{
  pub fn spawn(
    commands: &mut Commands,
    zoom: f32,
  ){
    commands.spawn((
      GameCamera,
      Camera2d::default(),
      Projection::Orthographic(OrthographicProjection {
        scale: zoom,
        ..OrthographicProjection::default_2d()
      }),
      tags::MainCamera,
      tags::GameEntity,
    ));
  }
}
