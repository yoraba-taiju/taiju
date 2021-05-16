use crate::prelude::*;
pub struct Bullet;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum BulletKind {
  BlueSmall,
  RedSmall,
  BlueNeedle,
  RedNeedle,
}
