use amethyst::{SimpleState, StateData, GameData, SimpleTrans, Trans};

pub struct LoadingState {
}

impl LoadingState {
  pub fn new() -> Self {
    Self {
    }
  }
}

impl SimpleState for LoadingState {
  fn fixed_update(&mut self, _data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
    Trans::None
  }
}
