use bevy::prelude::*;

use crate::logic::player;
use crate::logic::camera;



pub struct LogicPlugin;
impl Plugin for LogicPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins(player::PlayerLogicPlugin)
       .add_plugins(camera::CameraLogicPlugin);
  }
}
