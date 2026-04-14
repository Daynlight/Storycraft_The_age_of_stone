use bevy::prelude::*;

use crate::{components::{self, alt_movement::{MovementData, Velocity}}, systems::movement::PlayerInput};
use crate::components::tags::GameEntity;



pub const PLAYER_VELOCITY: f32 = 200.0;
pub const PLAYER_ACCELERATION: f32 = 1000.0;
pub const PLAYER_RESISTANCE: f32 = 5000.0;


#[derive(Component)]
pub struct Player;


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
      GameEntity,
      PlayerInput::default(),
      Velocity(Vec2::ZERO),
      MovementData::new(PLAYER_VELOCITY, PLAYER_ACCELERATION),
      components::movement::Movement::new(position, PLAYER_VELOCITY, PLAYER_ACCELERATION, PLAYER_RESISTANCE),
    ));
  }
}
