use bevy::prelude::*;

use crate::tags;
use crate::mechanics::collisions;



#[derive(Component)]
pub struct Counter;

impl Counter{
  pub fn spawn(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec2,
    size: Vec2,
  ){
    let texture = asset_server.load("Restaurant/Counter/Counter.png");
    commands.spawn((
      Sprite::from_image(texture), 
      tags::GameEntity,
      collisions::CollisionBox::new(Vec2::ZERO, size),
      Transform{
        translation: Vec3::new(position.x, position.y, 0.0),
        ..default()
      }
    ));
  }
}
