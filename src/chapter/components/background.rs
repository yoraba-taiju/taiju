use crate::prelude::*;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum BackgroundKind {
  Stage01,
}

impl BackgroundKind {
  fn spawn(&self, commands: &mut Commands) {
    match self {
      &BackgroundKind::Stage01 => {
      }
    }
  }
}