use bevy::prelude::*;
use std::time::Instant;

use crate::mechanics;



#[derive(Component, Default, Clone, Copy)]
pub struct CollisionBox{
  pub offset: Vec2,
  pub size: Vec2,
}


impl CollisionBox{
  pub fn new(offset: Vec2, size: Vec2) -> Self {
    CollisionBox{
      offset,
      size,
    }
  }

  pub fn intersects(
    self,
    pos_a: &Transform,
    pos_b: &Transform,
    second: &CollisionBox,
  ) -> bool {
    // estimate center
    let a_center = pos_a.translation.truncate() + self.offset;
    let b_center = pos_b.translation.truncate() + second.offset;

    let a_half = self.size / 2.0;
    let b_half = second.size / 2.0;

    let dx = (a_center.x - b_center.x).abs();
    let dy = (a_center.y - b_center.y).abs();

    dx <= (a_half.x + b_half.x) &&
    dy <= (a_half.y + b_half.y)
  }

  pub fn search_for_collisions(
    self,
    collision_boxes2: &Res<mechanics::collisions::systems::CollisionBoxesRegister>,
    entity: Entity,
    transform: &Transform,
  ) -> Vec<(Entity, Transform, mechanics::collisions::components::CollisionBox)> {
    let begging = Instant::now();

    let mut collisions: Vec<(Entity, Transform, mechanics::collisions::components::CollisionBox)> = Vec::new();

    for (entity2, transform2, collision_box2 ) in collision_boxes2.colliders_list.iter(){
      if entity != *entity2 {
        if self.intersects(transform, transform2, collision_box2) {
          collisions.push((*entity2, transform2.clone(), collision_box2.clone()));
        }
      }
    }

    let duration = begging.elapsed();
    info!("Collision Search: {:?}", duration);
    return collisions;
  }
}
