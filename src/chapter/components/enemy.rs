use crate::prelude::*;

pub struct Enemy;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum EnemyKind {
  Enemy01,
}
