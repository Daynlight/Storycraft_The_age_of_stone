use bevy::prelude::*;

use crate::components;



#[derive(Component)]
pub struct Player;
pub const PLAYER_VELOCITY: f32 = 200.0;
pub const PLAYER_ACCELERATION: f32 = 300.0;
pub const PLAYER_RESISTANCE: f32 = 600.0;


impl Player{
  pub fn spawn(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
  ){
    let texture = asset_server.load("Restaurant/Cook/Idle/Cook_1.png");
    let position = Vec2::new(64.0, 0.0);

    commands.spawn((
      Sprite::from_image(texture),
      Transform::from_xyz(64.0, 0.0, 0.0),
      Player,
      components::movement::Movement::new(position, PLAYER_VELOCITY, PLAYER_ACCELERATION, PLAYER_RESISTANCE),
    ));
  }
}
