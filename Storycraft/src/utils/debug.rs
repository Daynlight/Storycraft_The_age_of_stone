use bevy::prelude::*;

use crate::mechanics::{collisions, movement};
use crate::prefabs::player;
use crate::utils::utils;



fn draw_iso_box(
    gizmos: &mut Gizmos,
    center: Vec3,
    size: Vec3,
    color: Color,
) {
  let half = size * 0.5;

  let b1 = center + Vec3::new(-half.x, -half.y, -half.z);
  let b2 = center + Vec3::new( half.x, -half.y, -half.z);
  let b3 = center + Vec3::new( half.x, -half.y,  half.z);
  let b4 = center + Vec3::new(-half.x, -half.y,  half.z);
  let t1 = center + Vec3::new(-half.x,  half.y, -half.z);
  let t2 = center + Vec3::new( half.x,  half.y, -half.z);
  let t3 = center + Vec3::new( half.x,  half.y,  half.z);
  let t4 = center + Vec3::new(-half.x,  half.y,  half.z);

  let b1 = utils::world_to_view(b1).truncate();
  let b2 = utils::world_to_view(b2).truncate();
  let b3 = utils::world_to_view(b3).truncate();
  let b4 = utils::world_to_view(b4).truncate();
  let t1 = utils::world_to_view(t1).truncate();
  let t2 = utils::world_to_view(t2).truncate();
  let t3 = utils::world_to_view(t3).truncate();
  let t4 = utils::world_to_view(t4).truncate();

  gizmos.line_2d(b1, b2, color);
  gizmos.line_2d(b2, b3, color);
  gizmos.line_2d(b3, b4, color);
  gizmos.line_2d(b4, b1, color);
  gizmos.line_2d(t1, t2, color);
  gizmos.line_2d(t2, t3, color);
  gizmos.line_2d(t3, t4, color);
  gizmos.line_2d(t4, t1, color);
  gizmos.line_2d(b1, t1, color);
  gizmos.line_2d(b2, t2, color);
  gizmos.line_2d(b3, t3, color);
  gizmos.line_2d(b4, t4, color);
}


pub fn debug_colliders(
  mut gizmos: Gizmos,
  query: Query<(&movement::WorldPos, &collisions::CollisionBox)>
) {
  for (position, collider) in &query {
    let world_center = position.0 + collider.offset;

    draw_iso_box(
      &mut gizmos,
      world_center,
      collider.size,
      Color::linear_rgb(0.0, 1.0, 0.0),
    );
  }
}


pub fn debug_colliders_player(
  mut gizmos: Gizmos,
  query: Query<(&movement::WorldPos, &player::PlayerMovementCollider)>
) {
  for (position, collider) in &query {
    let world_center = position.0 + collider.offset;

    draw_iso_box(
      &mut gizmos,
      world_center,
      collider.size,
      Color::linear_rgb(1.0, 1.0, 0.0),
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