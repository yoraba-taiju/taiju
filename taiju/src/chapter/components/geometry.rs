use std::ops::{Add, Sub};

use crate::prelude::*;

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Position {
  pub x: f32,
  pub y: f32,
}

impl Position {
  pub const ZERO: Self = Self {x:0.0, y: 0.0};
  pub fn new(x: f32, y: f32) -> Self {
    Self {
      x,
      y,
    }
  }
  pub fn from_vec2(v: &Vec2) -> Self {
    Self {
      x: v.x,
      y: v.y,
    }
  }
  pub fn to_vec2(&self) -> Vec2 {
    Vec2::new(self.x, self.y)
  }
}

impl Sub for Position {
  type Output = Vec2;

  fn sub(self, rhs: Self) -> Self::Output {
    self.to_vec2() - rhs.to_vec2()
  }
}


#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Velocity {
  pub dx: f32,
  pub dy: f32,
}

impl Velocity {
  pub fn new(dx: f32, dy: f32) -> Self {
    Self {
      dx,
      dy,
    }
  }
  pub fn from_vec2(v: &Vec2) -> Self {
    Self {
      dx: v.x,
      dy: v.y,
    }
  }
  pub fn apply(&self, position: Position) -> Position {
    Position {
      x: position.x + self.dx,
      y: position.x + self.dy,
    }
  }
  pub fn to_vec2(&self) -> Vec2 {
    Vec2::new(self.dx, self.dy)
  }
}

/******************************************************************************
 ** Size/Area
 ******************************************************************************/

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Size {
  pub w: f32,
  pub h: f32,
}

impl Size {
  pub fn new(w: f32, h: f32) -> Self {
    Self {
      w,
      h,
    }
  }
  pub fn from_vec2(v: &Vec2) -> Self {
    Self {
      w: v.x,
      h: v.y,
    }
  }
  pub fn to_vec2(&self) -> Vec2 {
    Vec2::new(self.w, self.h)
  }
}

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Area {
  pub position: Position,
  pub size: Size,
}

impl Area {
  pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
    Self {
      position: Position::new(x, y),
      size: Size::new(w, h),
    }
  }
}

/******************************************************************************
 ** Rotation
 ******************************************************************************/

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Rotation {
  pub quaternion: Quat,
}

impl Rotation {
  pub fn new(z_angle: f32) -> Self {
    Self {
      quaternion: Quat::from_rotation_z(z_angle),
    }
  }
}

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RotationalVelocity {
  pub quaternion: Quat,
}

impl RotationalVelocity {
  pub fn new(z_angle: f32) -> Self {
    Self {
      quaternion: Quat::from_rotation_z(z_angle),
    }
  }
  pub fn apply(&self, rotation: &Rotation) -> Rotation {
    Rotation{
      quaternion: rotation.quaternion.mul_quat(self.quaternion).normalize(),
    }
  }
}
