use bevy::prelude::*;

use crate::utils::{tags, utils};
use crate::mechanics::collisions;
use crate::mechanics::movement;



#[derive(Component)]
pub struct Collider;

impl Collider{
  pub fn spawn(
    commands: &mut Commands,
    position: Vec3,
    size: Vec2,
  ){
    commands.spawn((
      collisions::CollisionBox::new(Vec2::ZERO, size),
      tags::GameEntity,
      movement::WorldPos(position),
      Transform{
        translation: utils::world_to_view(position),
        ..default()
      }
    ));
  }
}
