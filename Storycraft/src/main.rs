use bevy::prelude::*;

use crate::components::alt_movement::MovementPlugin;
use crate::scenes::scenes::{RegisteredScenes, SceneSystems, ActiveScene, LastScene};

mod config;
mod systems;
mod prefabs;
mod components;
mod scenes;



fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(SceneSystems::default())
    .insert_resource(ActiveScene::default())
    .insert_resource(LastScene::default())
    .add_systems(Startup, setup)
    .add_systems(Update, scene_swap)
    .add_systems(Update, systems::scenes::scene_system)
    .add_systems(PreUpdate, systems::movement::movement_system)
    .add_plugins(MovementPlugin)
    .run();
}


fn setup(
  mut commands: Commands,
  keyboard: Res<ButtonInput<KeyCode>>,
  mut active_scene: ResMut<ActiveScene>,
) {
  active_scene.0 = RegisteredScenes::Test;
}


fn scene_swap(
  mut commands: Commands,
  keyboard: Res<ButtonInput<KeyCode>>,
  mut active_scene: ResMut<ActiveScene>,
) {
  if keyboard.just_pressed(KeyCode::Digit1){
    active_scene.0 = RegisteredScenes::Test;
  }
  if keyboard.just_pressed(KeyCode::Digit2){
    active_scene.0 = RegisteredScenes::Test2;
  }
}

