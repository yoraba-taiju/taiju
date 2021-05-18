use crate::prelude::*;

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
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

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
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
  pub fn apply(&self, position: Position) -> Position {
    Position {
      x: position.x + self.dx,
      y: position.x + self.dy,
    }
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

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
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

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RotationalVelocity {
  pub quaternion: Quat,
}

impl RotationalVelocity {
  fn new(z_angle: f32) -> Self {
    Self {
      quaternion: Quat::from_rotation_z(z_angle),
    }
  }
  fn apply(&self, rotation: &Rotation) -> Rotation {
    Rotation{
      quaternion: rotation.quaternion.mul_quat(self.quaternion).normalize(),
    }
  }
}
