use crate::prelude::*;

pub struct Enemy(pub EnemyKind, pub EnemyAttackKind);

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum EnemyKind {
  Enemy01,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EnemyAttackKind {
  Attack01,
  Kamikaze,
}
