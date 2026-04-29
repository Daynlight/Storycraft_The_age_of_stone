use bevy::prelude::*;

mod config;
mod utils;
mod mechanics;
mod logic;
mod scenes;
mod prefabs;



fn main() {
  App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    // .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .add_systems(PreUpdate, scene_swap)
    .add_plugins(scenes::plugins::ScenePlugin)
    .add_plugins(mechanics::plugins::MechanicsPlugin)
    .add_plugins(logic::plugins::LogicPlugin)
    .add_plugins(utils::debug::DebugPlugin)
    .run();
}


fn setup(
  mut active_scene: ResMut<scenes::plugins::ActiveScene>,
) {
  active_scene.0 = scenes::register::ScenesRegister::Game;
}


fn scene_swap(
  keyboard: Res<ButtonInput<KeyCode>>,
  mut active_scene: ResMut<scenes::plugins::ActiveScene>,
) {
  if keyboard.just_pressed(KeyCode::Digit1){
    active_scene.0 = scenes::register::ScenesRegister::Game;
  }
}
