// use bevy::prelude::*;



// pub fn player_movement(
//   keyboard: Res<ButtonInput<KeyCode>>,
//   time: Res<Time>,
//   player_input: &mut PlayerInput,
//   player_movement: &mut components::movement::Movement,
//   player_transform: &mut Transform,
// ) {
//   let direction = get_direction_base_on_controls(keyboard).normalize_or_zero();
//   player_input.move_dir = direction;

//   // player_movement.make_move(direction, time.delta_secs());

//   let current_position = player_movement.get_current_position();
//   // player_transform.translation = Vec3::new(current_position.x, current_position.y, player_transform.translation.z);
// }


// pub fn movement_system(
//   keyboard: Res<ButtonInput<KeyCode>>,
//   time: Res<Time>,
//   camera: Single<&mut Transform, (With<MainCamera>, Without<Player>)>,
//   player: Single<(&mut Movement, &mut Transform, &mut PlayerInput), With<Player>>,
// ) {
//   let mut player_transform = player.into_inner();
//   let mut camera_transform = camera.into_inner();

//   player_movement(keyboard, time, &mut player_transform.2, &mut player_transform.0, &mut player_transform.1 );
//   update_camera_to_follow_player(&mut camera_transform, &player_transform.1);
// }


// fn set_player_velocity(
//   mut player: Single<(&PlayerInput, &MovementData, &mut Velocity)>
// ) {
//   let (player_input, settings, mut velocity) = player.into_inner();

//   let target = player_input.move_dir * settings.max_velocity;
//   let distance = target - velocity.0;

//   if distance.length() < settings.acceleration * DELTA_TIME {
//     velocity.0 = target;
//   } else {
//     let delta = distance.normalize_or_zero() * settings.acceleration * DELTA_TIME;
//     velocity.0 += delta;
//   }
// }



  // pub fn make_move(&mut self, direction: Vec2, delta_time: f32) {
  //   // [TODO] Energy loss at direction change relative to angle
  //   let direction = direction.normalize_or_zero();

  //   // resistance
  //   if self.current_velocity < 0.0{
  //     self.current_velocity = 0.0;
  //     self.direction = Vec2::ZERO;
  //     return;
  //   }

  //   if direction == Vec2::ZERO {
  //     if self.resistance == 0.0 {
  //       self.current_velocity -= self.resistance * delta_time;
  //     }
  //     else {
  //       self.current_velocity -= self.resistance * delta_time;
  //     }
  //   }

  //   // change direction resistance
  //   let angle = self.direction.perp_dot(direction);
  //   let energy_loss = ((angle / 5.0) * self.current_velocity).abs();
  //   self.current_velocity -= energy_loss;

  //   // acceleration
  //   if self.current_velocity > self.max_velocity {
  //     self.current_velocity = self.max_velocity;
  //   }

  //   if direction != Vec2::ZERO {
  //     self.current_velocity += self.acceleration * delta_time;
  //     self.direction = direction;
  //   }

  //   self.update_current_position(delta_time);
  // }
