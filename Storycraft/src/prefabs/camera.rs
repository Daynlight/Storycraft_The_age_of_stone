use bevy::prelude::*; 

use crate::config;
use crate::components::tags::GameEntity;



#[derive(Component)]
pub struct MainCamera;


impl MainCamera{
  pub fn spawn(
    commands: &mut Commands,
  ){
    commands.spawn((
      Camera2d::default(),
      MainCamera,
      GameEntity,
      Projection::Orthographic(OrthographicProjection {
        scale: config::settings::CAMERA_ZOOM,
        ..OrthographicProjection::default_2d()
      })
    ));
  }
}
