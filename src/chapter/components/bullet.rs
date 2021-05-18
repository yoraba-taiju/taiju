use crate::prelude::*;
pub struct Bullet(pub BulletKind, pub BulletAttackKind);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum BulletKind {
  BlueSmall,
  RedSmall,
  BlueNeedle,
  RedNeedle,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BulletAttackKind {
  Constant(Velocity)
}
