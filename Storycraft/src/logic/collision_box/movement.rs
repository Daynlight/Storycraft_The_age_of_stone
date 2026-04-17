use bevy::prelude::*;
use std::time::Instant;

use crate::config::benchmark;
use crate::tags;
use crate::mechanics;
use crate::scenes;



fn update_velocity(
  transform: &mut Transform,
  transform2: &Transform,
  box2: &mechanics::collisions::components::CollisionBox,
  velocity_vector: &mut mechanics::movement::components::EntityVelocityVector,
){
  let dx = transform.translation.x - transform2.translation.x;
  let px = box2.size.x - dx.abs();

  let dy = transform.translation.y - transform2.translation.y;
  let py = box2.size.y - dy.abs();

  let normal: Vec2;

  if px < py {
    normal = Vec2::new(dx.signum(), 0.0);
  } else {
    normal = Vec2::new(0.0, dy.signum());
  }

  velocity_vector.0 = (velocity_vector.0 - 2.0 * velocity_vector.0.dot(normal) * normal) * benchmark::ENERGY_LOSS;
  transform.translation += Vec3::new(normal.x, normal.y, 0.0);
}


fn set_collision_box_velocity(
  mut collision_boxes: Query<(Entity, &mut Sprite, &mut Transform, &mut mechanics::movement::components::EntityVelocityVector, &mechanics::collisions::components::CollisionBox), With<tags::CollisionBox>>,
  collision_boxes2: Res<mechanics::collisions::systems::CollisionBoxesRegister>,
) {
  let begging = Instant::now();

  for (entity, mut sprite, mut transform, mut velocity_vector, collision_box) in collision_boxes.iter_mut(){
    let collision_list = collision_box.search_for_collisions(&collision_boxes2, entity, &*transform);

    for (_, transform2, collision_box2) in collision_list.iter(){
      update_velocity(&mut transform, transform2, collision_box2, &mut velocity_vector);
      sprite.color = Color::linear_rgb(1.0, 0.0, 0.0);
    }
  }

  let duration = begging.elapsed();
  info!("Collision Calculations: {:?}", duration);
}


fn collision_box_movement_system_is_on(systems: Res<scenes::register::RunningSystemsRegister>) -> bool {
  systems.collision_box_movement
}


pub struct SetCollisionBoxMovementPlugin;
impl Plugin for SetCollisionBoxMovementPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(FixedPreUpdate, set_collision_box_velocity.run_if(collision_box_movement_system_is_on));
  }
}
