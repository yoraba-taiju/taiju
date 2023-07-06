use bevy::prelude::*;

fn main() -> anyhow::Result<()> {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system()
    .run();
  Ok(())
}
