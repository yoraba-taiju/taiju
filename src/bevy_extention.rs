use bevy::prelude::*;
mod scenario_loader;
#[derive(Default)]
pub struct TaijuPlugin;

impl Plugin for TaijuPlugin {
  fn build(&self, app: &mut AppBuilder) {
    { // Scenario Loader
      use scenario_loader::ScenarioLoader;
      use crate::chapter::scenario::Scenario;
      app
        .init_asset_loader::<ScenarioLoader>()
        .add_asset::<Scenario>();  
    }
  }
}
