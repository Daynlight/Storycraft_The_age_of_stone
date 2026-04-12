use bevy::prelude::*;








#[derive(Component)]
pub struct Player;
pub const PLAYER_VELOCITY: f32 = 200.0;


impl Player{
  pub fn spawn(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
  ){
    let texture = asset_server.load("Restaurant/Cook/Idle/Cook_1.png");

    commands.spawn((SpriteBundle {
      texture,
      transform: Transform::from_xyz(64.0, 0.0, 0.0),
      ..default()
    },
      Player
    ));
  }
}
