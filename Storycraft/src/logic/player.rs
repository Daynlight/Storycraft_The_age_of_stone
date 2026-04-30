use bevy::prelude::*;

use crate::config;
use crate::mechanics::{collisions, movement};
use crate::prefabs::player;
use crate::utils::{tags, utils};
use crate::scenes;



fn set_player_movement_data(
  keyboard: Res<ButtonInput<KeyCode>>,
  mut player_movement_data: Single<&mut movement::EntityMovementData, With<tags::MainPlayer>>,
) {
  let mut direction = Vec2::ZERO;

  if keyboard.any_pressed(config::UP) {
    direction.y = 1.0;
  }
  if keyboard.any_pressed(config::DOWN) {
    direction.y = -1.0;
  }
  if keyboard.any_pressed(config::LEFT) {
    direction.x = -1.0;
  }
  if keyboard.any_pressed(config::RIGHT) {
    direction.x = 1.0;
  }

  let dir: Vec3 = Vec3::new(direction.x, direction.y, 0.0).normalize_or_zero();
  let dir:Vec2 = utils::world_to_view_movement(dir).truncate();
  player_movement_data.movement_direction = dir;
}



pub struct PlayerMovementEventsPlugin;
impl Plugin for PlayerMovementEventsPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(PreUpdate, set_player_movement_data
      .run_if(|systems: Res<scenes::register::RunningSystemsRegister>| { systems.player_movement })
    );
  }
}


fn update_velocity(
  position: &mut Vec3,
  box1: collisions::CollisionBox,
  position2: Vec3,
  box2: collisions::CollisionBox,
  velocity_vector: &mut movement::EntityVelocityVector,
){
  let combined = box1.size + box2.size;
  let delta = (*position + box1.offset) - (position2 + box2.offset);
  let depth = combined / 2.0 - delta.abs();

  let normal = if depth.x < depth.y {
    Vec2::new(delta.x.signum(), 0.0)
  } else {
    Vec2::new(0.0, delta.y.signum())
  };

  velocity_vector.0 = (velocity_vector.0 - 2.0 * velocity_vector.0.dot(normal) * normal) * config::PLAYER_COLLISION_ENERGY_LOSS;

  position.x += normal.x * depth.x;
  position.y += normal.y * depth.y;
}


fn set_player_velocity(
  player: Single<(Entity, &mut movement::EntityVelocityVector, &mut movement::WorldPos, &movement::EntityMovementData, &player::PlayerMovementCollider), With<tags::MainPlayer>>,
  collision_boxes2: Res<collisions::CollisionBoxesRegister>,
) {
  let (entity, mut velocity_vector, mut position, entity_movement_data, collision_box) = player.into_inner();

  
  let direction = entity_movement_data.movement_direction;
  let target = direction * entity_movement_data.max_velocity;
  let distance = target - velocity_vector.0;
  
  if distance.length() < entity_movement_data.acceleration * config::FIXED_UPDATE_DELTA_TIME {
    velocity_vector.0 = target;
  } else {
    let delta = distance.normalize_or_zero() * entity_movement_data.acceleration * config::FIXED_UPDATE_DELTA_TIME;
    velocity_vector.0 += delta;
  }
  
  let collision_list = collision_box.0.search_for_collisions(entity, position.0, &collision_boxes2);
  for (position2, collision_box2) in collision_list.iter(){
    update_velocity(&mut position.0, collision_box.0, *position2, *collision_box2, &mut velocity_vector);
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


pub struct PlayerLogicPlugin;
impl Plugin for PlayerLogicPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins(PlayerMovementEventsPlugin)
       .add_plugins(SetPlayerMovementPlugin);
  }
}
