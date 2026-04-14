use bevy::prelude::*;

use crate::scenes::scenes::{ActiveScene, LastScene, SceneSystems, activate_scene};
use crate::components::tags::GameEntity;



pub fn scene_system(
  active_scene: Res<ActiveScene>,
  mut last_scene: ResMut<LastScene>,
  mut commands: Commands,
  mut systems: ResMut<SceneSystems>,
  query: Query<Entity, With<GameEntity>>,
  asset_server: Res<AssetServer>,
){
  if **active_scene == **last_scene {
    return;
  }

  *systems = SceneSystems::default();
  for entity in &query {
    commands.entity(entity).despawn();
  }

  activate_scene(**active_scene, commands, systems, asset_server);

  last_scene.0 = active_scene.0;
}