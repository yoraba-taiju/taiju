use crate::prelude::*;

pub struct Enemy(pub EnemyKind);

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum EnemyKind {
  Enemy01,
}
