use bevy::prelude::*;

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
    .add_systems(Update, systems::movement::movement_system.run_if(components::movement::movement_system_is_on))
    .add_plugins(components::alt_movement::MovementPlugin)
    .add_plugins(scenes::system::SceneResourcesPlugin)
    .add_plugins(scenes::register::RegisteredScenePlugin)
    .run();
}


fn setup(
  mut active_scene: ResMut<scenes::system::ActiveScene>,
) {
  active_scene.0 = scenes::register::RegisteredScenes::Test;
}


fn scene_swap(
  keyboard: Res<ButtonInput<KeyCode>>,
  mut active_scene: ResMut<scenes::system::ActiveScene>,
) {
  if keyboard.just_pressed(KeyCode::Digit1){
    active_scene.0 = scenes::register::RegisteredScenes::Test;
  }
  if keyboard.just_pressed(KeyCode::Digit2){
    active_scene.0 = scenes::register::RegisteredScenes::Test2;
  }
}
