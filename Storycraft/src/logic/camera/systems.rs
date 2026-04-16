use bevy::prelude::*;

use crate::logic::camera::tracking;



pub struct CameraLogicPlugin;
impl Plugin for CameraLogicPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins(tracking::CameraTrackingPlugin);
  }
}
