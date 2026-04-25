use bevy::prelude::*;

use crate::mechanics;
use crate::prefabs::player;



pub fn debug_colliders(
  mut gizmos: Gizmos,
  query: Query<(&Transform, &mechanics::collisions::CollisionBox)>
) {
  for (transform, collider) in &query {
    let center = transform.translation.truncate() + collider.offset;

    gizmos.rect_2d(
      center,
      collider.size,
      Color::linear_rgb(0.0, 1.0, 0.0),
    );
  }
}


pub fn debug_colliders_player(
  mut gizmos: Gizmos,
  query: Query<(&Transform, &player::PlayerMovementCollider)>
) {
  for (transform, collider) in &query {
    let center = transform.translation.truncate() + collider.offset;

    gizmos.rect_2d(
      center,
      collider.size,
      Color::linear_rgb(0.0, 1.0, 0.0),
    );
  }
}


pub struct DebugPlugin;
impl Plugin for DebugPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, debug_colliders)
       .add_systems(Update, debug_colliders_player);
  }
}