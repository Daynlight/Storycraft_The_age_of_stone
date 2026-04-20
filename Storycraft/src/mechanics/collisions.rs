use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::scenes;
use crate::config::settings;



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

  pub fn intersects(self, pos_a: Vec2, pos_b: Vec2, second: CollisionBox) -> bool {
    let a_center = pos_a + self.offset;
    let b_center = pos_b + second.offset;

    let a_half = self.size / 2.0;
    let b_half = second.size / 2.0;

    let delta_x = (a_center.x - b_center.x).abs();
    let delta_y = (a_center.y - b_center.y).abs();

    return delta_x <= (a_half.x + b_half.x) && delta_y <= (a_half.y + b_half.y);
  }

  pub fn search_for_collisions(
    self,
    entity: Entity,
    position: Vec2,
    collision_register: &CollisionBoxesRegister,
  ) -> Vec<(Vec2, CollisionBox)> {
    let mut collisions = Vec::new();
    collisions.reserve(settings::BUFFER_SIZE);

    let mut checked: HashSet<Entity> = HashSet::new();
    checked.reserve(settings::BUFFER_SIZE);

    let buckets = get_buckets(&self, &position);

    for bucket in buckets {
      if let Some(indices) = collision_register.colliders_buckets.get(&bucket) {
        for &i in indices {
          let (entity2, transform2, collision_box2) = &collision_register.colliders_vector[i];

          if *entity2 == entity {
            continue;
          }

          if !checked.insert(*entity2) {
            continue;
          }

          if self.intersects(position, *transform2, *collision_box2) {
            collisions.push((*transform2, *collision_box2));
          }
        }
      }
    }
    
    collisions.shrink_to_fit();
    return collisions;
  }
}



#[derive(Resource, Default)]
pub struct CollisionBoxesRegister{
  pub colliders_vector: Vec<(Entity, Vec2, CollisionBox)>,
  pub colliders_buckets: HashMap<IVec2, Vec<usize>>,
  pub entity_buckets: HashMap<Entity, Vec<IVec2>>,
  pub entity_index: HashMap<Entity, usize>,
}


fn get_buckets(
  collision_box: &CollisionBox,
  position: &Vec2,
) -> Vec<IVec2> {
  let mut buckets = Vec::new();
  buckets.reserve(settings::BUFFER_SIZE);

  let center = position + collision_box.offset;
  let half = collision_box.size / 2.0;

  let min = center - half;
  let max = center + half;

  let min_cell = (min / settings::BUCKETS_SIZE).floor().as_ivec2();
  let max_cell = (max / settings::BUCKETS_SIZE).floor().as_ivec2();

  for x in min_cell.x..=max_cell.x {
    for y in min_cell.y..=max_cell.y {
      buckets.push(IVec2::new(x, y));
    }
  }

  buckets.shrink_to_fit();
  return buckets;
}


fn remove_element_in_collision_register(
  collisions_register: &mut CollisionBoxesRegister,
  entity: Entity,
){
  if let Some(old_buckets) = collisions_register.entity_buckets.get(&entity) {
    if let Some(&index) = collisions_register.entity_index.get(&entity) {
      for bucket in old_buckets {
        if let Some(list) = collisions_register.colliders_buckets.get_mut(bucket) {
          list.retain(|&i| i != index);
        }
      }
    }
  }
}


fn add_element_to_collision_register(
  collisions_register: &mut CollisionBoxesRegister,
  entity: Entity,
  transform: &Transform,
  collision_box: &CollisionBox,
  new_buckets: Vec<IVec2>,
){
  if let Some(&index) = collisions_register.entity_index.get(&entity) {
    collisions_register.colliders_vector[index] = (entity, transform.translation.truncate(), *collision_box);

    for bucket in &new_buckets {
      collisions_register.colliders_buckets.entry(*bucket)
        .or_insert_with(|| { 
          let mut  v = Vec::new(); 
          v.reserve(settings::BUCKET_BUFFER_SIZE);
          v
        }).push(index);
    }
  } else {
    let index = collisions_register.colliders_vector.len();

    collisions_register.colliders_vector.push((entity, transform.translation.truncate(), *collision_box));

    for bucket in &new_buckets {
      collisions_register.colliders_buckets.entry(*bucket).or_insert_with(
        || { 
          let mut  v = Vec::new(); 
          v.reserve(settings::BUCKET_BUFFER_SIZE);
          v
        }).push(index);
    }

    collisions_register.entity_index.insert(entity, index);
  }

  collisions_register.entity_buckets.insert(entity, new_buckets);
}


fn update_element_in_collision_register(
  mut collisions_register: &mut CollisionBoxesRegister,
  entity: Entity,
  transform: &Transform,
  collision_box: &CollisionBox,
) {
  let new_buckets = get_buckets(collision_box, &transform.translation.truncate());

  remove_element_in_collision_register(&mut collisions_register, entity);
  add_element_to_collision_register(collisions_register, entity, transform, collision_box, new_buckets);
}


fn generate_collision_data(
  mut collisions_register: ResMut<CollisionBoxesRegister>,
  collisions: Query<(Entity, &Transform, &CollisionBox), Changed<Transform>>
) {
  for (entity, transform, collision_box) in collisions.iter() {
    update_element_in_collision_register(&mut collisions_register, entity, transform, collision_box);
  }
}


fn removed_collision_data(
  mut removed: RemovedComponents<Transform>,
  mut collisions_register: ResMut<CollisionBoxesRegister>,
) {
  for entity in &mut removed.read() {
    remove_element_in_collision_register(&mut collisions_register, entity);
  }
}


fn added_collision_data(
  mut collisions_register: ResMut<CollisionBoxesRegister>,
  added: Query<(Entity, &Transform, &CollisionBox), Added<CollisionBox>>
) {
  for (entity, transform, collision_box) in added {
    let new_buckets = get_buckets(collision_box, &transform.translation.truncate());
    add_element_to_collision_register(&mut collisions_register, entity, transform, collision_box, new_buckets);
  }
}



pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
  fn build(&self, app: &mut App) {
    app.insert_resource(CollisionBoxesRegister::default())
      .add_systems(FixedPreUpdate, (generate_collision_data, removed_collision_data, added_collision_data)
        .run_if(|systems: Res<scenes::register::RunningSystemsRegister>| { systems.collisions })
      );
  }
}
