use bevy::prelude::*; 

use crate::config;
use crate::tags;
use crate::prefabs::camera;



#[derive(Component)]
pub struct Camera2D;


impl Camera2D{
  pub fn spawn(
    commands: &mut Commands,
  ){
    commands.spawn((
      Camera2d::default(),
      camera::Camera2D,
      tags::MainCamera,
      tags::GameEntity,
      Projection::Orthographic(OrthographicProjection {
        scale: config::settings::CAMERA_ZOOM,
        ..OrthographicProjection::default_2d()
      })
    ));
  }
}
