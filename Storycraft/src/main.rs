use bevy::prelude::*;









fn main() {
  let mut app: ::bevy::app::App = ::bevy::app::App::new();
  app.add_plugins(DefaultPlugins);
  app.add_systems(Startup, setup);
  app.add_systems(Update, camera_move);
  app.add_systems(Update, camera_zoom);
  app.run();
}



#[derive(Component)]
struct MainCamera;

fn setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
) {
  // add Camera
  commands.spawn((Camera2dBundle::default(), MainCamera));

  // add Counter
  let texture = asset_server.load("Restaurant/Counter/Counter.png");
  commands.spawn(SpriteBundle {
    texture,
    transform: Transform::from_xyz(0.0, 0.0, 0.0),
    ..default()
  });

  commands.spawn(SpriteBundle {
    sprite: Sprite {
      color: Color::rgb(1.0, 0.0, 0.0),
      custom_size: Some(Vec2::new(32.0, 32.0)),
      ..default()
    },
    transform: Transform::from_xyz(64.0, 0.0, 0.0),
    ..default()
  });
}




const CAMERA_VELOCITY: f32 = 200.0;

fn camera_move(
  keyboard: Res<ButtonInput<KeyCode>>,
  time: Res<Time>,
  mut query: Query<&mut Transform, With<MainCamera>>,
) {
  let speed = CAMERA_VELOCITY * time.delta_seconds();

  for mut transform in &mut query {
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

const CAMERA_ZOOM_SPEED: f32 = 1.3;
const CAMERA_ZOOM_RANGE: Vec2 = Vec2::new(0.1, 2.0);

fn camera_zoom(
  keyboard: Res<ButtonInput<KeyCode>>,
  time: Res<Time>,
  mut query: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
  let delta = CAMERA_ZOOM_SPEED * time.delta_seconds();

  for mut projection in &mut query {
    if keyboard.pressed(KeyCode::KeyP) {
      projection.scale -= delta;
    }

    if keyboard.pressed(KeyCode::KeyL) {
      projection.scale += delta;
    }

    projection.scale = projection.scale.clamp(CAMERA_ZOOM_RANGE.x, CAMERA_ZOOM_RANGE.y);
  }
}