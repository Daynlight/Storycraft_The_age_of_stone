use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::scenes;
use crate::config;
use crate::mechanics::movement;



#[derive(Component, Default, Clone, Copy)]
pub struct CollisionBox{
  pub offset: Vec3,
  pub size: Vec3,
}


impl CollisionBox{
  pub fn new(offset: Vec3, size: Vec3) -> Self {
    CollisionBox{
      offset,
      size,
    }
  }

  pub fn intersects(self, second: CollisionBox, position_a: Vec3, position_b: Vec3) -> bool {
    let boxes_size: Vec3 = (self.size + second.size) / 2.0;
    let delta: Vec3 = (position_a + self.offset - position_b - second.offset).abs();

    return delta.x <= boxes_size.x && delta.y <= boxes_size.y && delta.z <= boxes_size.z;
  }

  pub fn search_for_collisions(
    self,
    entity: Entity,
    position: Vec3,
    collision_register: &CollisionBoxesRegister,
  ) -> Vec<(Vec3, CollisionBox)> {
    let mut collisions: Vec<(Vec3, CollisionBox)> = Vec::new();
    collisions.reserve(config::BUFFER_SIZE);

    let mut checked: HashSet<Entity> = HashSet::new();
    checked.reserve(config::BUFFER_SIZE);

    let buckets: Vec<IVec3> = get_buckets(&self, &position).1;

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
  position: &Vec3,
) -> (IVec3, Vec<IVec3>) {
  let mut buckets: Vec<IVec3> = Vec::new();
  buckets.reserve(config::BUFFER_SIZE);

  let center: Vec3 = position + collision_box.offset;
  let half: Vec3 = collision_box.size / 2.0;

  let min: Vec3 = center - half;
  let max: Vec3 = center + half;

  let min_cell: IVec3 = (min / config::BUCKETS_SIZE).floor().as_ivec3();
  let max_cell: IVec3 = (max / config::BUCKETS_SIZE).floor().as_ivec3();

  for x in min_cell.x..=max_cell.x {
    for y in min_cell.y..=max_cell.y {
      for z in min_cell.z..=max_cell.z {
       buckets.push(IVec3::new(x, y, z));
      }
    }
  }

  buckets.shrink_to_fit();
  return (IVec3::new(min_cell.x - max_cell.x, min_cell.x - max_cell.x, min_cell.z - max_cell.z), buckets);
}



#[derive(Resource, Default)]
pub struct CollisionBoxesRegister{
  pub colliders_vector: Vec<(Entity, Vec3, CollisionBox)>,
  pub colliders_buckets: HashMap<IVec3, Vec<usize>>,
  pub entity_buckets: HashMap<Entity, (IVec3, Vec<IVec3>)>,
  pub entity_vector_index: HashMap<Entity, usize>,
}


impl CollisionBoxesRegister{
  fn buckets_are_different(
    &self,
    entity: Entity,
    new_buckets: &(IVec3, Vec<IVec3>),
  ) -> bool {
    if let Some(old_buckets) = self.entity_buckets.get(&entity) {
      if old_buckets.1.len() != new_buckets.1.len() {
        return true;
      }

      if old_buckets.0.x != new_buckets.0.x || old_buckets.0.y != new_buckets.0.y {
        return true;
      }

      if !new_buckets.1[0] != old_buckets.1[0] {
        return true;
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
    position: &Vec3,
    collision_box: &CollisionBox,
    new_buckets: &(IVec3, Vec<IVec3>),
  ){
    if let Some(&index) = self.entity_vector_index.get(&entity) {
      for bucket in &new_buckets.1 {
        self.colliders_buckets.entry(*bucket)
          .or_insert_with(|| { 
            let mut  v = Vec::new(); 
            v.reserve(config::BUCKET_BUFFER_SIZE);
            v
          }).push(index);
      }
    } else {
      let index = self.colliders_vector.len();

      self.colliders_vector.push((entity, *position, *collision_box));

      for bucket in &new_buckets.1 {
        self.colliders_buckets.entry(*bucket).or_insert_with(
          || { 
            let mut  v = Vec::new(); 
            v.reserve(config::BUCKET_BUFFER_SIZE);
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
        for bucket in &old_buckets.1 {
          if let Some(bucket_data) = self.colliders_buckets.get_mut(&bucket) {
            bucket_data.retain(|&i| i != index);
            
            if bucket_data.len() <= 0 {
              self.colliders_buckets.remove(&bucket);
            }
          }
        }
      }
    }
  }


  fn add_update_entity(
    &mut self,
    entity: Entity,
    position: &movement::WorldPos,
    collision_box: &CollisionBox,
  ){
    if let Some(&index) = self.entity_vector_index.get(&entity) {
      self.colliders_vector[index] = (entity, position.0, *collision_box);
    }
    else{
      let new_index = self.colliders_vector.len();
      self.colliders_vector.push((entity, position.0, *collision_box));
      self.entity_vector_index.insert(entity, new_index);
    }
  }


  fn remove_entity(
    &mut self,
    entity: Entity,
  ){
    self.remove_entity_from_buckets(entity);

    // if self.colliders_vector.len() <= 1{
    //   self.entity_vector_index.remove(&entity);
    //   self.entity_buckets.remove(&entity);
    //   self.colliders_vector.pop();
    //   return;
    // }
    
    // let last_index = self.colliders_vector.len() - 1;
    // let last_entity = self.colliders_vector[last_index];

    // self.remove_entity_from_buckets(last_entity.0);

    // if let Some(&index) = self.entity_vector_index.get(&entity) {
    //   self.colliders_vector[index] = last_entity;
    //   self.entity_vector_index.insert(last_entity.0, index);
    // }

    // self.entity_vector_index.remove(&entity);
    // self.entity_buckets.remove(&entity);
    // self.colliders_vector.pop();

    // let new_buckets = get_buckets(&last_entity.2, &last_entity.1);
    // self.add_update_entity_to_buckets(last_entity.0, &last_entity.1, &last_entity.2, &new_buckets);
  }
}



fn update_collision_system(
  mut collisions_register: ResMut<CollisionBoxesRegister>,
  collisions: Query<(Entity, &movement::WorldPos, &CollisionBox), Changed<movement::WorldPos>>
) {
  for (entity, position, collision_box) in collisions.iter() {
    let new_buckets = get_buckets(collision_box, &position);

    if !collisions_register.buckets_are_different(entity, &new_buckets){
      collisions_register.add_update_entity(entity, position, collision_box);
      continue;
    }
    collisions_register.remove_entity_from_buckets(entity);
    collisions_register.add_update_entity(entity, position, collision_box);
    collisions_register.add_update_entity_to_buckets(entity, &position, collision_box, &new_buckets);
  }
}


fn added_collision_system(
  mut collisions_register: ResMut<CollisionBoxesRegister>,
  added: Query<(Entity, &movement::WorldPos, &CollisionBox), Added<CollisionBox>>
) {
  for (entity, position, collision_box) in added {
    let new_buckets = get_buckets(collision_box, &position);

    if !collisions_register.buckets_are_different(entity, &new_buckets){
      collisions_register.add_update_entity(entity, position, collision_box);
      continue;
    }
    
    collisions_register.remove_entity_from_buckets(entity);
    collisions_register.add_update_entity(entity, position, collision_box);
    collisions_register.add_update_entity_to_buckets(entity, &position, collision_box, &new_buckets);
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
