use bevy::prelude::*;

use crate::utils::{tags, utils};
use crate::mechanics::{collisions, movement};



#[derive(Component)]
pub struct Collider;

impl Collider{
    pub fn from_range(
    commands: &mut Commands,
    start: IVec3,
    end: IVec3,
  ){
    let size = (start.as_vec3() - end.as_vec3()).abs() + Vec3::new(1.0, 1.0, 1.0);
    let position = start.as_vec3() + (size - Vec3::new(1.0, 1.0, 1.0)) / 2.0;
    Collider::spawn(commands, position, size);
  }

  pub fn spawn(
    commands: &mut Commands,
    position: Vec3,
    size: Vec3,
  ){
    commands.spawn((
      Collider,
      Transform{
        translation: utils::world_to_view(position),
        ..default()
      },
      collisions::CollisionBox::new(Vec3::ZERO, size),
      movement::WorldPos(position),
      tags::GameEntity,
    ));
  }
}
