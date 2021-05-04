use bevy::prelude::*;
use crate::scenes::stage::prelude::*;

pub mod components;
pub mod resources;
pub mod prelude;

#[derive(Default)]
pub struct StagePlugin;

impl Plugin for StagePlugin {
  fn build(&self, app: &mut AppBuilder) {
    use crate::scenes::stage::scenario::*;
    app
      .init_asset_loader::<ScenarioLoader>()
      .add_asset::<Scenario>();
  }
}

pub struct StageScene {
}

impl StageScene {
  pub fn new() -> Self {
    Self {
    }
  }
}

pub fn setup(
  mut commands: Commands,
  clock: Res<ClockRef>,
  mut color_materials: ResMut<Assets<ColorMaterial>>,
  asset_server: Res<AssetServer>,
) {
  commands.insert_resource(ScenarioSequencer::spawn(&clock, &asset_server));
  // witches
  Sora::spawn(&clock, &mut commands, &asset_server, &mut color_materials);

  // cameras
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
  commands.spawn_bundle(UiCameraBundle::default());
}