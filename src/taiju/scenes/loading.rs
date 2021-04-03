use amethyst::{SimpleState, StateData, GameData, SimpleTrans, Trans};
use crate::taiju::scenes::stage::StageState;

pub struct LoadingState {
}

impl LoadingState {
  pub fn new() -> Self {
    Self {
    }
  }
}

impl SimpleState for LoadingState {
  fn fixed_update(&mut self, _data: StateData<'_, GameData>) -> SimpleTrans {
    Trans::Replace(Box::new(StageState::new()))
  }
}
