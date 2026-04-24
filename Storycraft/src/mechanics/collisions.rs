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

  pub fn intersects(self, second: CollisionBox, position_a: Vec2, position_b: Vec2) -> bool {
    let boxes_size: Vec2 = (self.size + second.size) / 2.0;
    let delta: Vec2 = (position_a + self.offset - position_b - second.offset).abs();

    return delta.x <= boxes_size.x && delta.y <= boxes_size.y;
  }

  pub fn search_for_collisions(
    self,
    entity: Entity,
    position: Vec2,
    collision_register: &CollisionBoxesRegister,
  ) -> Vec<(Vec2, CollisionBox)> {
    let mut collisions: Vec<(Vec2, CollisionBox)> = Vec::new();
    collisions.reserve(settings::BUFFER_SIZE);

    let mut checked: HashSet<Entity> = HashSet::new();
    checked.reserve(settings::BUFFER_SIZE);

    let buckets: Vec<IVec2> = get_buckets(&self, &position);

    for bucket in buckets {
      if let Some(indices) = collision_register.colliders_buckets.get(&bucket) {
        for &index in indices {
          let (entity2, position2, collision_box2) = &collision_register.colliders_vector[index];

          if *entity2 == entity {
            continue;
          }

          if !checked.insert(*entity2) {
            continue;
          }

          if self.intersects(*collision_box2, position, *position2) {
            collisions.push((*position2, *collision_box2));
          }
        }
      }
    }
    
    collisions.shrink_to_fit();
    return collisions;
  }
}


fn get_buckets(
  collision_box: &CollisionBox,
  position: &Vec2,
) -> Vec<IVec2> {
  let mut buckets: Vec<IVec2> = Vec::new();
  buckets.reserve(settings::BUFFER_SIZE);

  let center: Vec2 = position + collision_box.offset;
  let half: Vec2 = collision_box.size / 2.0;

  let min: Vec2 = center - half;
  let max: Vec2 = center + half;

  let min_cell: IVec2 = (min / settings::BUCKETS_SIZE).floor().as_ivec2();
  let max_cell: IVec2 = (max / settings::BUCKETS_SIZE).floor().as_ivec2();

  for x in min_cell.x..=max_cell.x {
    for y in min_cell.y..=max_cell.y {
      buckets.push(IVec2::new(x, y));
    }
  }

  buckets.shrink_to_fit();
  return buckets;
}



#[derive(Resource, Default)]
pub struct CollisionBoxesRegister{
  pub colliders_vector: Vec<(Entity, Vec2, CollisionBox)>,
  pub colliders_buckets: HashMap<IVec2, Vec<usize>>,
  pub entity_buckets: HashMap<Entity, Vec<IVec2>>,
  pub entity_vector_index: HashMap<Entity, usize>,
}


impl CollisionBoxesRegister{
  fn buckets_are_different(
    &self,
    entity: Entity,
    new_buckets: &Vec<IVec2>,
  ) -> bool {
    if let Some(old_buckets) = self.entity_buckets.get(&entity) {
      if old_buckets.len() != new_buckets.len() {
        return true;
      }

      for bucket in old_buckets {
        if !new_buckets.contains(bucket) {
          return true;
        }
      }
    }
    else{
      return true;
    }
    
    return false;
  }


  fn add_update_entity_to_buckets(
    &mut self,
    entity: Entity,
    position: &Vec2,
    collision_box: &CollisionBox,
    new_buckets: &Vec<IVec2>,
  ){
    if let Some(&index) = self.entity_vector_index.get(&entity) {
      for bucket in new_buckets {
        self.colliders_buckets.entry(*bucket)
          .or_insert_with(|| { 
            let mut  v = Vec::new(); 
            v.reserve(settings::BUCKET_BUFFER_SIZE);
            v
          }).push(index);
      }
    } else {
      let index = self.colliders_vector.len();

      self.colliders_vector.push((entity, *position, *collision_box));

      for bucket in new_buckets {
        self.colliders_buckets.entry(*bucket).or_insert_with(
          || { 
            let mut  v = Vec::new(); 
            v.reserve(settings::BUCKET_BUFFER_SIZE);
            v
          }).push(index);
      }

      self.entity_vector_index.insert(entity, index);
    }

    self.entity_buckets.insert(entity, new_buckets.clone());
  }


  fn remove_entity_from_buckets(
    &mut self,
    entity: Entity,
  ){
    if let Some(old_buckets) = self.entity_buckets.get(&entity) {
      if let Some(&index) = self.entity_vector_index.get(&entity) {
        for bucket in old_buckets {
          if let Some(bucket_data) = self.colliders_buckets.get_mut(bucket) {
            bucket_data.retain(|&i| i != index);
            
            if bucket_data.len() <= 0 {
              self.colliders_buckets.remove(bucket);
            }
          }
        }
      }
    }
  }


  fn add_update_entity(
    &mut self,
    entity: Entity,
    transform: &Transform,
    collision_box: &CollisionBox,
  ){
    if let Some(&index) = self.entity_vector_index.get(&entity) {
      self.colliders_vector[index] = (entity, transform.translation.truncate(), *collision_box);
    }
    else{
      let new_index = self.colliders_vector.len();
      self.colliders_vector.push((entity, transform.translation.truncate(), *collision_box));
      self.entity_vector_index.insert(entity, new_index);
    }
  }


  fn remove_entity(
    &mut self,
    entity: Entity,
  ){
    self.remove_entity_from_buckets(entity);

    if self.colliders_vector.len() <= 1{
      self.entity_vector_index.remove(&entity);
      self.entity_buckets.remove(&entity);
      self.colliders_vector.pop();
      return;
    }
    
    let last_index = self.colliders_vector.len() - 1;
    let last_entity = self.colliders_vector[last_index];

    self.remove_entity_from_buckets(last_entity.0);

    if let Some(&index) = self.entity_vector_index.get(&entity) {
      self.colliders_vector[index] = last_entity;
      self.entity_vector_index.insert(last_entity.0, index);
    }

    self.entity_vector_index.remove(&entity);
    self.entity_buckets.remove(&entity);
    self.colliders_vector.pop();

    let new_buckets = get_buckets(&last_entity.2, &last_entity.1);
    self.add_update_entity_to_buckets(last_entity.0, &last_entity.1, &last_entity.2, &new_buckets);
  }
}



fn update_collision_system(
  mut collisions_register: ResMut<CollisionBoxesRegister>,
  collisions: Query<(Entity, &Transform, &CollisionBox), Changed<Transform>>
) {
  for (entity, transform, collision_box) in collisions.iter() {
    let new_buckets = get_buckets(collision_box, &transform.translation.truncate());

    if !collisions_register.buckets_are_different(entity, &new_buckets){
      collisions_register.add_update_entity(entity, transform, collision_box);
      continue;
    }
    collisions_register.remove_entity_from_buckets(entity);
    collisions_register.add_update_entity(entity, transform, collision_box);
    collisions_register.add_update_entity_to_buckets(entity, &transform.translation.truncate(), collision_box, &new_buckets);
  }
}


fn added_collision_system(
  mut collisions_register: ResMut<CollisionBoxesRegister>,
  added: Query<(Entity, &Transform, &CollisionBox), Added<CollisionBox>>
) {
  for (entity, transform, collision_box) in added {
    let new_buckets = get_buckets(collision_box, &transform.translation.truncate());

    if !collisions_register.buckets_are_different(entity, &new_buckets){
      collisions_register.add_update_entity(entity, transform, collision_box);
      continue;
    }
    
    collisions_register.remove_entity_from_buckets(entity);
    collisions_register.add_update_entity(entity, transform, collision_box);
    collisions_register.add_update_entity_to_buckets(entity, &transform.translation.truncate(), collision_box, &new_buckets);
  }
}


fn removed_collision_system(
  mut removed: RemovedComponents<Transform>,
  mut collisions_register: ResMut<CollisionBoxesRegister>,
) {
  for entity in &mut removed.read() {
    collisions_register.remove_entity_from_buckets(entity);
    collisions_register.remove_entity(entity);
  }
}


pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
  fn build(&self, app: &mut App) {
    app.insert_resource(CollisionBoxesRegister::default())
      .add_systems(FixedPreUpdate, ((removed_collision_system, added_collision_system, update_collision_system).chain())
        .run_if(|systems: Res<scenes::register::RunningSystemsRegister>| { systems.collisions })
      );
  }
}
