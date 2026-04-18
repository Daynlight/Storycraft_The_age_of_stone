use bevy::prelude::*;

use crate::logic::collision_box::movement;



pub struct CollisionBoxLogicPlugin;
impl Plugin for CollisionBoxLogicPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins(movement::SetCollisionBoxMovementPlugin);
  }
}
