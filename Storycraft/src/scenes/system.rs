use bevy::prelude::*;

use crate::scenes::register;



#[derive(Resource, Deref, Default)]
pub struct ActiveScene(pub register::RegisteredScenes);


#[derive(Resource, Deref, Default)]
pub struct LastScene(pub register::RegisteredScenes);


pub struct SceneResourcesPlugin;
impl Plugin for SceneResourcesPlugin {
  fn build(&self, app: &mut App) {
    app.insert_resource(register::SceneSystemsRegister::default())
    .insert_resource(ActiveScene::default())
    .insert_resource(LastScene::default());
  }
}
