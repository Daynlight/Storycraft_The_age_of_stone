use bevy::prelude::*;

mod camera;
use camera::{MainCamera, camera_move_system};

mod player;
use player::{Player, player_move_system};








fn main() {
  let mut app: ::bevy::app::App = ::bevy::app::App::new();
  app.add_plugins(DefaultPlugins);
  app.add_systems(Startup, setup);
  app.add_systems(Update, player_move_system);
  app.add_systems(Update, camera_move_system);
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
