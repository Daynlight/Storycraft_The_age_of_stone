use bevy::prelude::*;

use crate::scenes::{scene, systems};



#[derive(Resource, Default)]
pub struct RunningSystemsRegister{
  pub movement: bool,
  pub player_events: bool,
  pub camera_tracking: bool,
  pub player_movement: bool,
  pub collision_box_movement: bool,
  pub generate_collision: bool,
}


#[derive(PartialEq, Default, Clone, Copy)]
pub enum ScenesRegister{
  #[default]
  Null,
  Game,
  CollisionBenchmark,
}


pub struct RegisteredScenePlugin;
impl Plugin for RegisteredScenePlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, (
      scene::game::set.run_if(scene::game::check),
      scene::collision_benchmark::set.run_if(scene::collision_benchmark::check)
    ).run_if(systems::scene_changed));
  }
}
