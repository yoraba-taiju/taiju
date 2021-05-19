use crate::prelude::*;
pub struct EnemyBullet(pub EnemyBulletKind, pub EnemyBulletAttackKind);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum EnemyBulletKind {
  BlueSmall,
  RedSmall,
  BlueNeedle,
  RedNeedle,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EnemyBulletAttackKind {
  Constant(Velocity)
}
