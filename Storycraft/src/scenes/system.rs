use bevy::prelude::*;

use crate::scenes::register;



#[derive(Resource, Deref, Default)]
pub struct ActiveScene(pub register::ScenesRegister);


#[derive(Resource, Deref, Default)]
pub struct LastScene(pub register::ScenesRegister);


pub fn scene_changed(active: Res<ActiveScene>, last: Res<LastScene>) -> bool {
  **active != **last
}


fn update_last_scene_system(active: Res<ActiveScene>, mut last: ResMut<LastScene>){ 
  last.0 = active.0;
}


pub struct ScenePlugin;
impl Plugin for ScenePlugin {
  fn build(&self, app: &mut App) {
    app.insert_resource(register::RunningSystemsRegister::default())
    .insert_resource(ActiveScene::default())
    .insert_resource(LastScene::default())
    .add_plugins(register::RegisteredScenePlugin)
    .add_systems(PostUpdate, update_last_scene_system);
  }
}
