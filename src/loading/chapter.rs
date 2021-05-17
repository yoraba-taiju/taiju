use crate::{
  chapter::resources::{
    WitchServer,
    EnemyServer,
    BulletServer,
  },
  prelude::*
};
use crate::AppState;

pub fn on_enter<'a, 'b, 'c>(
  mut commands: Commands<'a>,
  asset_server: Res<'b, AssetServer>,
  color_materials: ResMut<'c, Assets<ColorMaterial>>,
) {
  let mut loader = Loader::new(asset_server, color_materials);
  let scenario_server = ScenarioSever::from_loader(&mut loader);
  let witch_server = WitchServer::from_loader(&mut loader);
  let enemy_server = EnemyServer::from_loader(&mut loader);
  let bullet_server = BulletServer::from_loader(&mut loader);
  commands.insert_resource(scenario_server);
  commands.insert_resource(witch_server);
  commands.insert_resource(enemy_server);
  commands.insert_resource(bullet_server);
  let status: LoadingStatus = loader.build();
  commands.insert_resource(status);
}

pub fn update(
  mut state: ResMut<State<AppState>>,
  asset_server: Res<AssetServer>,
  status: Res<LoadingStatus>,
) {
  if !status.is_loaded(&asset_server) {
    return;
  }

  state.set(AppState::InChapter).unwrap();
}

pub fn on_exit(
  mut commands: Commands,
) {
  commands.remove_resource::<LoadingStatus>();
}
