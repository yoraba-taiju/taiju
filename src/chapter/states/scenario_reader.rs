use crate::prelude::*;

use crate::chapter::scenario::*;
#[derive(Debug)]
pub struct  ScenarioReader {
  pub scenario: Scenario,
  pub page: Value<u32>,
}

impl ScenarioReader {
  fn new(clock: ClockRef, scenario: Scenario) -> Self {
    Self {
      scenario,
      page: clock.make(0),
    }
  }
}