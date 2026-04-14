use bevy::prelude::*;

use crate::scenes::scene;



#[derive(Resource, Default)]
pub struct SceneSystemsRegister{
  pub movement: bool,
}


#[derive(PartialEq, Default, Clone, Copy)]
pub enum RegisteredScenes{
  #[default]
  Null,
  Test,
  Test2,
}


pub struct RegisteredScenePlugin;
impl Plugin for RegisteredScenePlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, (
      scene::test::set,
      scene::test2::set
    ).chain());
  }
}
