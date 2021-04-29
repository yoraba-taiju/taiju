pub mod witches;
pub mod background;

pub use witches::*;
pub use background::*;
use crate::donut::Value;

#[derive(Debug)]
pub struct Position {
  pub x: Value<f32>,
  pub y: Value<f32>,
  pub w: Value<f32>,
  pub h: Value<f32>,
}
