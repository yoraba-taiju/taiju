use crate::prelude::*;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Size {
  pub w: f32,
  pub h: f32,
}

impl Size {
  fn new(w: f32, h: f32) -> Self {
    Self {
      w,
      h,
    }
  }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Position {
  pub x: f32,
  pub y: f32,
}

impl Position {
  pub fn new(x: f32, y: f32) -> Self {
    Self {
      x,
      y,
    }
  }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Velocity {
  pub x: f32,
  pub y: f32,
}

impl Velocity {
  pub fn new(x: f32, y: f32) -> Self {
    Self {
      x,
      y,
    }
  }
}


#[derive(Default, Debug, Clone, Serialize, Deserialize)]
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

#[derive(Default, Debug, Clone, Copy)]
pub struct Rotation {
  pub quaternion: Quat,
}

impl Rotation {
  fn new(z_angle: f32) -> Self {
    Self {
      quaternion: Quat::from_rotation_z(z_angle),
    }
  }
}