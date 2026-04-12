use bevy::prelude::*;








#[derive(Component)]
pub struct MainCamera;
const CAMERA_ZOOM: f32 = 0.5;


impl MainCamera{
  pub fn spawn(
    commands: &mut Commands,
  ){
    let mut camera: Camera2dBundle = Camera2dBundle::default();
    camera.projection.scale = CAMERA_ZOOM;

    commands.spawn((camera, MainCamera));
  }
}
