use bevy::prelude::*;

use crate::mechanics::player_events::components;
use crate::config;
use crate::scenes;



fn get_player_movement_data(
  keyboard: Res<ButtonInput<KeyCode>>,
  mut player_movement_data: ResMut<components::PlayerMovementData> 
) {
  let mut direction = Vec2::ZERO;

  if keyboard.any_pressed(config::controls::UP) {
    direction.y = 1.0;
  }
  if keyboard.any_pressed(config::controls::DOWN) {
    direction.y = -1.0;
  }
  if keyboard.any_pressed(config::controls::LEFT) {
    direction.x = -1.0;
  }
  if keyboard.any_pressed(config::controls::RIGHT) {
    direction.x = 1.0;
  }

  player_movement_data.movement_direction = direction;
}


fn player_events_system_is_on(systems: Res<scenes::register::RunningSystemsRegister>) -> bool {
  systems.player_events
}


pub struct PlayerEventsPlugin;
impl Plugin for PlayerEventsPlugin {
  fn build(&self, app: &mut App) {
    app.insert_resource(components::PlayerMovementData::default())
    .add_systems(PreUpdate, get_player_movement_data.run_if(player_events_system_is_on));
  }
}
