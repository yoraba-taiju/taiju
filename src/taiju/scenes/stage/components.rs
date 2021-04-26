pub mod witches;
pub mod background;

pub use witches::*;
pub use background::*;

#[derive(Default, Debug)]
pub struct Position {
  pub x: f32,
  pub y: f32,
  pub w: f32,
  pub h: f32,
}
