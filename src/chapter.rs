use crate::prelude::*;

pub mod components;
pub mod states;
pub mod resources;
pub mod system;
pub mod scenario;

pub fn on_enter(
  mut commands: Commands,
  scenario_server: Res<ScenarioSever>,
  scenarios: Res<Assets<Scenario>>,
  asset_server: Res<AssetServer>,
  mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
  // Init clock
  let clock: ClockRef = Clock::new();
  commands.insert_resource(clock.clone());

  // Span Scenario Reader
  let scenario = scenario_server.get_scenario(&scenarios);
  commands.insert_resource(ScenarioReader::new(clock.clone(), scenario));

  // cameras
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
  commands.spawn_bundle(UiCameraBundle::default());
}

pub fn on_exit(
  mut commands: Commands,
) {
  // Remove State resources
  commands.remove_resource::<ScenarioReader>();

  // Remove Servers
  commands.remove_resource::<WitchServer>();
  commands.remove_resource::<EnemyServer>();
  commands.remove_resource::<BulletServer>();
  commands.remove_resource::<ScenarioSever>();

  // Remove Clock
  commands.remove_resource::<ClockRef>();
}
