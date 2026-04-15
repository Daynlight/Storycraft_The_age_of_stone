use bevy::prelude::*;

use crate::scenes::{self, scene, system};



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
      scene::test::set.run_if(scene::test::in_test_scene),
      scene::test2::set.run_if(scene::test2::in_test2_scene)
    ).run_if(system::scene_change))
    .add_systems(PostUpdate,scenes::system::update_last_scene_system);
  }
}
