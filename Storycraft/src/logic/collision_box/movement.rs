use bevy::prelude::*;

use crate::config::benchmark;
use crate::tags;
use crate::scenes;
use crate::mechanics::{collisions, movement};



fn update_velocity(
  transform: &mut Transform,
  transform2: Vec2,
  box2: &collisions::CollisionBox,
  velocity_vector: &mut movement::EntityVelocityVector,
){
  let delta = transform.translation.truncate() - transform2;
  let depth = box2.size - delta.abs();

  let normal = if depth.x < depth.y { Vec2::new(delta.x.signum(), 0.0) } else { Vec2::new(0.0, delta.y.signum()) };

  velocity_vector.0 = (velocity_vector.0 - 2.0 * velocity_vector.0.dot(normal) * normal) * benchmark::ENERGY_LOSS;
  transform.translation += Vec3::new(normal.x, normal.y, 0.0);
}


fn set_collision_box_velocity(
  mut collision_boxes: Query<(Entity, &mut Sprite, &mut Transform, &mut movement::EntityVelocityVector, &collisions::CollisionBox), With<tags::CollisionBox>>,
  collision_boxes2: Res<collisions::CollisionBoxesRegister>,
) {
  for (entity, mut sprite, mut transform, mut velocity_vector, collision_box) in collision_boxes.iter_mut(){
    let collision_list = collision_box.search_for_collisions(entity, transform.translation.truncate(), &collision_boxes2);
    
    for (transform2, collision_box2) in collision_list.iter(){
      update_velocity(&mut transform, *transform2, collision_box2, &mut velocity_vector);
      sprite.color = Color::linear_rgb(1.0, 0.0, 0.0);
    }
  }
}



pub struct SetCollisionBoxMovementPlugin;
impl Plugin for SetCollisionBoxMovementPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(FixedPreUpdate, set_collision_box_velocity.
      run_if(|systems: Res<scenes::register::RunningSystemsRegister>| { systems.collisions })
    );
  }
}
