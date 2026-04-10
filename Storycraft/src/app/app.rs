use bevy::prelude::*;










pub struct App {
  bevy_app: ::bevy::app::App,
}


impl App {
  pub fn new() -> Self {
    let mut bevy_app = ::bevy::app::App::new();

    bevy_app.add_plugins(DefaultPlugins);
    bevy_app.add_systems(Startup, Self::setup);

    return App{ bevy_app };
  }

  pub fn run(mut self) {
    self.bevy_app.run();
  }

  fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
  }
}
