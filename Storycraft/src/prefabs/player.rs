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
    let position = position + Vec3::new(0.0, 0.0, 0.5);

    commands.spawn((
      Player,
      Sprite{
        image: texture,
        custom_size: Some(config::SPRITE_SIZE),
        ..default()
      },
      Transform{
        translation: utils::world_to_view(position),
        ..default()
      },
      mechanics::movement::WorldPos(position),
      PlayerMovementCollider(mechanics::collisions::CollisionBox::new(Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.5, 0.5, 0.1))),
      // PlayerHitBoxCollider(mechanics::collisions::CollisionBox::new(Vec2::ZERO, Vec2::new(32.0, 64.0))),
      mechanics::movement::EntityMovementData::new(config::PLAYER_VELOCITY, config::PLAYER_ACCELERATION),
      mechanics::movement::EntityVelocityVector::default(),
      tags::MainPlayer,
      tags::GameEntity,
    ));
  }
}
