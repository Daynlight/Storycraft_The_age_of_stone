use bevy::prelude::*;

use crate::mechanics::movement;
use crate::config;
use crate::scenes;
use crate::tags;



fn set_player_movement_data(
  keyboard: Res<ButtonInput<KeyCode>>,
  mut player_movement_data: Single<&mut movement::EntityMovementData, With<tags::MainPlayer>>,
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

  player_movement_data.movement_direction = direction.normalize_or_zero();
}



pub struct PlayerMovementEventsPlugin;
impl Plugin for PlayerMovementEventsPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(PreUpdate, set_player_movement_data
      .run_if(|systems: Res<scenes::register::RunningSystemsRegister>| { systems.player_movement })
    );
  }
}
