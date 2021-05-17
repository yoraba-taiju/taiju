use crate::prelude::*;

use crate::chapter::scenario::Scenario;

pub struct ScenarioSever {
  handle: Handle<Scenario>,
}

impl ScenarioSever {
  pub fn from_loader(
    loader: &mut Loader
  ) -> Self {
    Self {
      handle: loader.load::<Scenario>("scenario/stage01.ron")
    }
  }

  pub fn get_scenario(&self, scenarios: &Res<Assets<Scenario>>) -> Scenario {
    scenarios.get(&self.handle).unwrap().clone()
  }
}
