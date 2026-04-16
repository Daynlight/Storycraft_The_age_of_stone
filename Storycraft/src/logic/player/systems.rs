use bevy::prelude::*;

use crate::logic::player::events;
use crate::logic::player::movement;


pub struct PlayerLogicPlugin;
impl Plugin for PlayerLogicPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins(events::PlayerMovementEventsPlugin)
    .add_plugins(movement::SetPlayerMovementPlugin);
  }
}
