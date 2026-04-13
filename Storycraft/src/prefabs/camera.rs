use bevy::prelude::*; 

use crate::config;



#[derive(Component)]
pub struct MainCamera;


impl MainCamera{
  pub fn spawn(
    commands: &mut Commands,
  ){
    commands.spawn((
      Camera2d,
      MainCamera,
      Projection::Orthographic(OrthographicProjection {
        scale: config::settings::CAMERA_ZOOM,
        ..OrthographicProjection::default_2d()
      })
    ));
  }
}
