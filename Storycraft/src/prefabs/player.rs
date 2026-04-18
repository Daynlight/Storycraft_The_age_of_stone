use bevy::prelude::*;

use crate::mechanics;
use crate::tags;
use crate::config::settings;



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
      Player,
      tags::MainPlayer,
      tags::GameEntity,
      mechanics:: collisions::CollisionBox::new(Vec2::ZERO, Vec2::new(32.0, 32.0)),
      mechanics::movement::EntityVelocityVector::default(),
      mechanics::movement::EntityMovementData::new(settings::PLAYER_VELOCITY, settings::PLAYER_ACCELERATION),
    ));
  }
}
