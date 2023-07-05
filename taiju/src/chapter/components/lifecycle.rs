use crate::prelude::*;

#[derive(Debug)]
pub struct Spawned {
  pub at: u32,
}

impl Spawned {
  pub(crate) fn new(clock: &ClockRef) ->Self {
    Self {
      at: clock.current_time().ticks,
    }
  }
}

#[derive(Debug)]
pub struct Vanished {
  pub at: u32,
}

impl Vanished {
  pub(crate) fn new(clock: &ClockRef) ->Self {
    Self {
      at: clock.current_time().ticks,
    }
  }
}
