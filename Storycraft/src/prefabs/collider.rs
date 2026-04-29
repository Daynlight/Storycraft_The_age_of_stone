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
    let mut end = end + end.signum();
    if end.x == 0 { end.x = 1 };
    if end.y == 0 { end.y = 1 };
    if end.z == 0 { end.z = 1 };

    let size = (start - end).abs();
    let position = (start + end).as_vec3() / 2.0;
    Collider::spawn(commands, position, size.as_vec3());
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
