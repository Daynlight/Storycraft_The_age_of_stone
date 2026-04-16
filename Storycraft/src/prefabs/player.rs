use bevy::prelude::*;

use crate::tags;
use crate::config::settings;
use crate::mechanics::movement;



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
      Transform::from_xyz(position.x, position.y, 0.0),
      tags::MainPlayer,
      tags::GameEntity,
      movement::components::EntityVelocityVector::default(),
      movement::components::EntityMovementData::new(settings::PLAYER_VELOCITY, settings::PLAYER_ACCELERATION),
    ));
  }
}
