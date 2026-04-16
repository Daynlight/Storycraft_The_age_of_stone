use bevy::prelude::*; 

use crate::config;
use crate::tags;



#[derive(Component)]
pub struct GameCamera;


impl GameCamera{
  pub fn spawn(
    commands: &mut Commands,
  ){
    commands.spawn((
      Camera2d::default(),
      GameCamera,
      tags::MainCamera,
      tags::GameEntity,
      Projection::Orthographic(OrthographicProjection {
        scale: config::settings::CAMERA_ZOOM,
        ..OrthographicProjection::default_2d()
      })
    ));
  }
}
