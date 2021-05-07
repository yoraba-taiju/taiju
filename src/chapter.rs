use bevy::asset::LoadState;
use bevy::prelude::*;

use crate::chapter::prelude::*;
use crate::app::AppState;

pub mod components;
pub mod resources;
pub mod prelude;

#[derive(Default)]
pub struct StagePlugin;

impl Plugin for StagePlugin {
  fn build(&self, app: &mut AppBuilder) {
    use crate::chapter::scenario::*;
    app
      .init_asset_loader::<ScenarioLoader>()
      .add_asset::<Scenario>();
  }
}

pub fn setup(
  mut commands: Commands,
  clock: Res<ClockRef>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  mut color_materials: ResMut<Assets<ColorMaterial>>,
  asset_server: Res<AssetServer>,
) {
  // Resources to load
  ScenarioSever::spawn(&mut commands, &clock, &asset_server);
  EnemyServer::spawn(&mut commands, &asset_server, &mut color_materials, &mut texture_atlases);
  BulletServer::spawn(&mut commands, &asset_server, &mut color_materials, &mut texture_atlases);

  // witches
  Sora::spawn(&mut commands, &clock, &asset_server, &mut color_materials);

  // cameras
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
  commands.spawn_bundle(UiCameraBundle::default());
}

pub fn check_setup(
  mut state: ResMut<State<AppState>>,
  asset_server: Res<AssetServer>,
  scenario_server: Res<ScenarioSever>,
  enemy_server: Res<EnemyServer>,
  bullet_server: Res<BulletServer>,
) {
  if LoadState::Loaded != asset_server.get_group_load_state(scenario_server.get_asset_handles()) {
    return;
  }
  // TODO: handle texture loading correctly (or not).
  if LoadState::Loaded != asset_server.get_group_load_state(enemy_server.get_asset_handles()) {
    //return;
  }
  if LoadState::Loaded != asset_server.get_group_load_state(bullet_server.get_asset_handles()) {
    //return;
  }
  state.set(AppState::Stage).unwrap();
}
