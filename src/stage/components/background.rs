use crate::stage::prelude::*;

pub enum BackgroundType {
  Stage01,
}

impl BackgroundType {
  fn spawn(&self, commands: &mut Commands) {
    match self {
      &BackgroundType::Stage01 => {
      }
    }
  }
}