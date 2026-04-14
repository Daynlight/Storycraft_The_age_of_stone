use bevy::prelude::*;

use crate::scenes::register;
use crate::scenes::resources;



pub struct ScenePlugin;
impl Plugin for ScenePlugin {
  fn build(&self, app: &mut App) {
    app.insert_resource(register::SceneSystemsRegister::default())
    .insert_resource(resources::ActiveScene::default())
    .insert_resource(resources::LastScene::default());
  }
}
