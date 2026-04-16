use bevy::prelude::*;

mod config;
mod prefabs;
mod logic;
mod mechanics;
mod scenes;
mod tags;



fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .add_systems(PreUpdate, scene_swap)
    .add_plugins(mechanics::systems::MechanicsPlugin)
    .add_plugins(scenes::systems::ScenePlugin)
    .add_plugins(logic::systems::LogicPlugin)
    .run();
}


fn setup(
  mut active_scene: ResMut<scenes::systems::ActiveScene>,
) {
  active_scene.0 = scenes::register::ScenesRegister::Game;
}


fn scene_swap(
  keyboard: Res<ButtonInput<KeyCode>>,
  mut active_scene: ResMut<scenes::systems::ActiveScene>,
) {
  if keyboard.just_pressed(KeyCode::Digit1){
    active_scene.0 = scenes::register::ScenesRegister::Game;
  }
  if keyboard.just_pressed(KeyCode::Digit2){
    active_scene.0 = scenes::register::ScenesRegister::CollisionBenchmark;
  }
}
