use bevy::prelude::*;

mod config;
mod systems;
mod prefabs;
mod components;



fn main() {
  let mut app: ::bevy::app::App = ::bevy::app::App::new();
  app.add_plugins(DefaultPlugins);
  app.add_systems(Startup, setup);
  app.add_systems(Update, systems::movement::movement_system);
  app.run();
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
