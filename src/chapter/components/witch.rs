use crate::prelude::*;

pub struct Witch {
  pub health: Value<u16>,
  pub spell: Value<u16>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum WitchKind {
  Sora,
  Chitose,
  Momiji,
  Kaede,
}

pub struct Sora;

pub struct Chitose;

pub struct Momiji;

pub struct Kaede;
