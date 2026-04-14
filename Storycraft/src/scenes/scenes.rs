use bevy::prelude::*;

use crate::scenes;
use crate::components::tags::GameEntity;



#[derive(Resource, Default)]
pub struct SceneSystems{
  pub movement: bool,
}


#[derive(PartialEq, Default, Clone, Copy)]
pub enum RegisteredScenes{
  #[default]
  Null,
  Test,
  Test2,
}


#[derive(Resource, Deref, Default)]
pub struct ActiveScene(pub RegisteredScenes);


#[derive(Resource, Deref, Default)]
pub struct LastScene(pub RegisteredScenes);


pub fn activate_scene(
  scene: RegisteredScenes,
  mut commands: Commands,
  mut systems: ResMut<SceneSystems>,
  asset_server: Res<AssetServer>,
){
  if scene == RegisteredScenes::Test{
    scenes::test::set(commands, systems, asset_server);
  }
  else if scene == RegisteredScenes::Test2{
    scenes::test2::set(commands, systems, asset_server);
  }
}
