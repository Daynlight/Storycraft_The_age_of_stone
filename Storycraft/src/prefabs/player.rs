use bevy::prelude::*;

use crate::mechanics;
use crate::utils::{tags, utils};
use crate::config;



#[derive(Component, Deref)]
pub struct PlayerMovementCollider(pub mechanics::collisions::CollisionBox);

// #[derive(Component)]
// pub struct PlayerHitBoxCollider(pub mechanics::collisions::CollisionBox);


#[derive(Component)]
pub struct Player;


impl Player{
  pub fn spawn(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
  ){
    let texture = asset_server.load("Placeholders/Player.png");

    commands.spawn((
      Sprite{
        image: texture,
        custom_size: Some(Vec2::new(32.0, 32.0)),
        ..default()
      },
      Transform{
        translation: utils::world_to_view(position),
        ..default()
      },
      Player,
      tags::MainPlayer,
      tags::GameEntity,
      mechanics::movement::WorldPos(position),
      PlayerMovementCollider(mechanics::collisions::CollisionBox::new(Vec2::new(0.0, -13.0), Vec2::new(14.0, 5.0))),
      // PlayerHitBoxCollider(mechanics::collisions::CollisionBox::new(Vec2::ZERO, Vec2::new(32.0, 64.0))),
      mechanics::movement::EntityMovementData::new(config::PLAYER_VELOCITY, config::PLAYER_ACCELERATION),
      mechanics::movement::EntityVelocityVector::default(),
    ));
  }
}
