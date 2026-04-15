use bevy::prelude::*;

use crate::prefabs;
use crate::scenes;



fn camera_tracking(
  mut camera: Single<&mut Transform, (With<prefabs::camera::MainCamera>, Without<prefabs::player::Player>)>,
  player: Single<&Transform, With<prefabs::player::Player>>,
) {
  camera.translation = player.translation;
}


fn camera_tracking_system_is_on(systems: Res<scenes::register::RunningSystemsRegister>) -> bool {
  systems.camera_tracking
}


pub struct CameraTrackingPlugin;
impl Plugin for CameraTrackingPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(PostUpdate, camera_tracking.run_if(camera_tracking_system_is_on));
  }
}
