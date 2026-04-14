use bevy::prelude::*;

use crate::components::alt_movement::MovementPlugin;
use crate::scenes::scenes::{RegisteredScenes, ActiveScene};
use crate::systems::scenes::ScenePlugin;

mod config;
mod systems;
mod prefabs;
mod components;
mod scenes;



fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .add_systems(Update, scene_swap)
    .add_systems(Update, systems::movement::movement_system)
    .add_plugins(MovementPlugin)
    .add_plugins(ScenePlugin)
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
