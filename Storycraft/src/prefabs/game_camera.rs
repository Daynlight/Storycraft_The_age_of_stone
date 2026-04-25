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
      Camera2d::default(),
      GameCamera,
      tags::MainCamera,
      tags::GameEntity,
      Projection::Orthographic(OrthographicProjection {
        scale: zoom,
        ..OrthographicProjection::default_2d()
      })
    ));
  }
}
