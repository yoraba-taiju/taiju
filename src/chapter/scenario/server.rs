use bevy::prelude::*;
use super::*;
pub struct ScenarioSever {
  scenario_handle: Handle<Scenario>,
}

impl ScenarioSever {
  pub fn spawn(
    commands: &mut Commands,
    clock: &Res<ClockRef>,
    asset_server: &Res<AssetServer>,
  ) {
    let handle = asset_server.load::<Scenario, _>("scenario/stage01.ron");
    commands.insert_resource(ScenarioSever{
      scenario_handle: handle,
    });
  }
  pub fn get_asset_handles(&self) -> Vec<HandleId> {
    vec![self.scenario_handle.id]
  }
}
