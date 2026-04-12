use bevy::prelude::*;

mod systems;
use systems::movement::movement_system;

mod camera;
use camera::{MainCamera};

mod player;
use player::{Player};








fn main() {
  let mut app: ::bevy::app::App = ::bevy::app::App::new();
  app.add_plugins(DefaultPlugins);
  app.add_systems(Startup, setup);
  app.add_systems(Update, movement_system);
  app.run();
}


fn setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
) {
  MainCamera::spawn(&mut commands);
  Player::spawn(&mut commands, &asset_server);

  // add Counter
  let texture = asset_server.load("Restaurant/Counter/Counter.png");
  commands.spawn(SpriteBundle {
    texture,
    transform: Transform::from_xyz(0.0, 0.0, 0.0),
    ..default()
  });
}
