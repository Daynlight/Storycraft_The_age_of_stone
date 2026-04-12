use bevy::prelude::*;








#[derive(Component)]
pub struct Player;
const PLAYER_VELOCITY: f32 = 200.0;


impl Player{
  pub fn spawn(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
  ){
    let texture = asset_server.load("Restaurant/Cook/Idle/Cook_1.png");

    commands.spawn((SpriteBundle {
      texture,
      transform: Transform::from_xyz(64.0, 0.0, 0.0),
      ..default()
    },
      Player
    ));
  }
}



pub fn player_move_system(
  keyboard: Res<ButtonInput<KeyCode>>,
  time: Res<Time>,
  mut player_query: Query<&mut Transform, With<Player>>,
) {
  let speed = PLAYER_VELOCITY * time.delta_seconds();

  for mut transform in &mut player_query {
    if keyboard.pressed(KeyCode::KeyW) {
      transform.translation.y += speed;
    }
    if keyboard.pressed(KeyCode::KeyS) {
      transform.translation.y -= speed;
    }
    if keyboard.pressed(KeyCode::KeyA) {
      transform.translation.x -= speed;
    }
    if keyboard.pressed(KeyCode::KeyD) {
      transform.translation.x += speed;
    }
  }
}
