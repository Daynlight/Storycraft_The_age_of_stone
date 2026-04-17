use bevy::prelude::*;

use crate::tags;
use crate::mechanics::collisions;



#[derive(Component)]
pub struct StaticCollisionBox;


impl StaticCollisionBox{
  pub fn spawn(
    commands: &mut Commands,
    position: Vec2,
    size: Vec2,
  ){
    commands.spawn((
      Sprite{
        color: Color::linear_rgb(0.0, 0.5, 0.5),
        custom_size: Some(size),
        ..default()
      },
      StaticCollisionBox,
      tags::GameEntity,
      tags::CollisionBox,
      collisions::CollisionBox::new(Vec2::ZERO, size),
      Transform::from_xyz(position.x, position.y, 0.0)
    ));
  }
}
