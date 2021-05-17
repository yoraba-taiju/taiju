use crate::prelude::*;

pub struct EnemyAttack(pub EnemyAttackKind);
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum EnemyAttackKind {
  Attack01,
  Kamikaze,
}
