use bevy::prelude::*;
use std::time::Instant;

use crate::scenes;
use crate::mechanics::collisions;



#[derive(Resource, Default)]
pub struct CollisionBoxesRegister{
  pub colliders_list: Vec<(Entity, Transform, collisions::components::CollisionBox)>,
}


fn generate_collision_data(
  mut collisions_register: ResMut<CollisionBoxesRegister>,
  collisions: Query<(Entity, &Transform, &collisions::components::CollisionBox)>
) {
  let begging = Instant::now();

  collisions_register.colliders_list.clear();
  for (entity, transform, collision_box) in collisions.iter() {
    collisions_register.colliders_list.push((entity, transform.clone(), collision_box.clone()));
  }

  let duration = begging.elapsed();
  info!("Collision Update: {:?}", duration);
}


fn generate_collision_system_is_on(systems: Res<scenes::register::RunningSystemsRegister>) -> bool {
  systems.generate_collision
}


pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
  fn build(&self, app: &mut App) {
    app.insert_resource(CollisionBoxesRegister::default())
       .add_systems(FixedPreUpdate, generate_collision_data.run_if(generate_collision_system_is_on));
  }
}
