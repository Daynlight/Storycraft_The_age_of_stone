use bevy::prelude::*;

use crate::tags;
use crate::mechanics::movement;



#[derive(Component)]
pub struct CollisionBox;


impl CollisionBox{
  pub fn spawn(
    commands: &mut Commands,
    position: Vec2,
    direction: Vec2,
  ){
    commands.spawn((
      Sprite{
        color: Color::linear_rgb(0.0, 0.5, 0.5),
        custom_size: Some(Vec2::new(10.0, 10.0)),
        ..default()
      }, tags::GameEntity,
      movement::components::EntityVelocityVector(direction),
      Transform::from_xyz(position.x, position.y, 0.0)
    ));
  }
}
