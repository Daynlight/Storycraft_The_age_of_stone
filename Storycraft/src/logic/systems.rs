use bevy::prelude::*;

use crate::logic::player;
use crate::logic::camera;
use crate::logic::collision_box;



pub struct LogicPlugin;
impl Plugin for LogicPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins(player::systems::PlayerLogicPlugin)
    .add_plugins(camera::systems::CameraLogicPlugin)
    .add_plugins(collision_box::systems::CollisionBoxLogicPlugin);
  }
}
