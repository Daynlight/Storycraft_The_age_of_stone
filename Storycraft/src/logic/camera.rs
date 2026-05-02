use bevy::prelude::*;

use crate::utils::tags;
use crate::scenes;



fn camera_tracking(
  mut camera: Single<&mut Transform, (With<tags::MainCamera>, Without<tags::MainPlayer>)>,
  player: Single<&Transform, With<tags::MainPlayer>>,
) {
  camera.translation = player.translation;
}


fn camera_tracking_system_is_on(systems: Res<scenes::register::RunningSystemsRegister>) -> bool {
  systems.camera_tracking
}


pub struct CameraLogicPlugin;
impl Plugin for CameraLogicPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(PostUpdate, camera_tracking.run_if(camera_tracking_system_is_on));
  }
}