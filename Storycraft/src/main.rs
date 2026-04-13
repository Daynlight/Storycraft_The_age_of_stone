use bevy::prelude::*;

use crate::components::alt_movement::MovementPlugin;

mod config;
mod systems;
mod prefabs;
mod components;



fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .add_systems(PreUpdate, systems::movement::movement_system)
    .add_plugins(MovementPlugin)
    .run();
}


fn setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
) {
  prefabs::camera::MainCamera::spawn(&mut commands);
  prefabs::player::Player::spawn(&mut commands, &asset_server);

  // add Counter
  let texture = asset_server.load("Restaurant/Counter/Counter.png");
  commands.spawn((
    Sprite::from_image(texture),
    Transform::from_xyz(0.0, 0.0, 0.0)
  ));
}
