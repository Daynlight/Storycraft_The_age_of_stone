use bevy::prelude::*;

use crate::config::settings;
use crate::mechanics::{collisions, movement};
use crate::tags;
use crate::scenes;



fn update_velocity(
  transform: &mut Transform,
  box1: collisions::CollisionBox,
  position2: Vec2,
  box2: collisions::CollisionBox,
  velocity_vector: &mut movement::EntityVelocityVector,
){
  let combined = box1.size + box2.size;
  let delta = transform.translation.truncate() - position2;
  let depth = combined / 2.0 - delta.abs();

  let normal = if depth.x < depth.y { Vec2::new(delta.x.signum(), 0.0) } else { Vec2::new(0.0, delta.y.signum()) };

  velocity_vector.0 = (velocity_vector.0 - 2.0 * velocity_vector.0.dot(normal) * normal) * settings::PLAYER_COLLISION_ENERGY_LOSS;

  transform.translation += Vec3::new(normal.x * depth.x, normal.y * depth.y, 0.0);
}


fn set_player_velocity(
  player: Single<(Entity, &mut movement::EntityVelocityVector, &mut Transform, &movement::EntityMovementData, &collisions::CollisionBox), With<tags::MainPlayer>>,
  collision_boxes2: Res<collisions::CollisionBoxesRegister>,
) {
  let (entity, mut velocity_vector, mut transform, entity_movement_data, collision_box) = player.into_inner();

  
  let direction = entity_movement_data.movement_direction.normalize_or_zero();
  let target = direction * entity_movement_data.max_velocity;
  let distance = target - velocity_vector.0;
  
  if distance.length() < entity_movement_data.acceleration * settings::FIXED_UPDATE_DELTA_TIME {
    velocity_vector.0 = target;
  } else {
    let delta = distance.normalize_or_zero() * entity_movement_data.acceleration * settings::FIXED_UPDATE_DELTA_TIME;
    velocity_vector.0 += delta;
  }
  
  let collision_list = collision_box.search_for_collisions(entity, transform.translation.truncate(), &collision_boxes2);
  for (position2, collision_box2) in collision_list.iter(){
    update_velocity(&mut transform, *collision_box, *position2, *collision_box2, &mut velocity_vector);
  }
}



pub struct SetPlayerMovementPlugin;
impl Plugin for SetPlayerMovementPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(PreUpdate, set_player_velocity
      .run_if(|systems: Res<scenes::register::RunningSystemsRegister>| { systems.player_movement })
    );
  }
}
