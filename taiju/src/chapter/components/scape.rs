use super::geometry::*;
use serde::{Serialize, Deserialize};
#[derive(Debug)]
pub struct Scape {
  pub size: (f32, f32),
  pub hit_area: Vec<Area>,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ScapeKind {
  Scape01,
}
