use bevy::prelude::*;



#[derive(Component)]
pub struct MainCamera;
const CAMERA_ZOOM: f32 = 0.5;


impl MainCamera{
  pub fn spawn(
    commands: &mut Commands,
  ){
    commands.spawn((
      Camera2d,
      MainCamera,
      Projection::Orthographic(OrthographicProjection {
        scale: CAMERA_ZOOM,
        ..OrthographicProjection::default_2d()
      })
    ));
  }
}
