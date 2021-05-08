use crate::chapter::prelude::*;

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Size {
  pub w: f32,
  pub h: f32,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Position {
  pub x: f32,
  pub y: f32,
}

impl Position {
  pub fn apply(&mut self, motion: &Motion) {
    match motion {
      &Motion::Constant(x, y) => {
        self.x += x;
        self.y += y;
      }
    }
  }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Motion {
  Constant(f32, f32),
}

impl Default for Motion {
  fn default() -> Self {
    Motion::Constant(0.0, 0.0)
  }
}

pub fn move_by_motion(clock: Res<ClockRef>, mut query: Query<(&mut Value<Position>, &Motion)>) {
  if clock.is_inspected() {
    return;
  }
  for (mut pos, motion) in query.iter_mut() {
    let pos: &mut Position = &mut pos;
    let motion: &Motion = &motion;
    pos.apply(motion);
  }
}

pub fn copy_pos_to_transform(mut query: Query<(&Value<Position>, &mut Transform)>) {
  for (pos, mut trans) in query.iter_mut() {
    let pos: &Value<Position> = &pos;
    let trans: &mut Transform = &mut trans;
    trans.translation.x = pos.x;
    trans.translation.y = pos.y;
  }
}

pub fn handle_entity_vanishing(
  mut commands: Commands,
  clock: Res<ClockRef>,
  mut query: Query<(Entity, &Value<Position>), (Without<Vanished>, Without<Witch>)>
) {
  if(clock.is_inspected()) {
    return;
  }
  for (entity, pos) in query.iter_mut() {
    let entity: Entity = entity;
    let pos: &Value<Position> = &pos;
    let x = pos.x.clone();
    let y = pos.y.clone();
    if x < (-30.0-(1920.0/2.0)) ||
      (30.0+(1920.0/2.0)) < x ||
      y < (-30.0-(1080.0/2.0)) ||
      (30.0+(1080.0/2.0)) < y {
      commands.entity(entity).insert(Vanished::new(&clock));
    }
  }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Rotation {
  pub quaternion: Quat,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AngularMotion {
  Constant(f32),
}

impl Default for AngularMotion {
  fn default() -> Self {
    AngularMotion::Constant(Default::default())
  }
}

pub fn copy_rotation_to_transform(mut query: Query<(&Value<Rotation>, &mut Transform)>) {
  for (rot, mut trans) in query.iter_mut() {
    let rot: &Value<Rotation> = &rot;
    let trans: &mut Transform = &mut trans;
    trans.rotation = rot.quaternion;
  }
}

pub fn move_by_angular_motion(_clock: Res<ClockRef>, mut query: Query<(&mut Value<Rotation>, &AngularMotion)>) {
  for (mut rot, motion) in query.iter_mut() {
    let rot: &mut Value<Rotation> = &mut rot;
    let motion: &AngularMotion = &motion;
    match motion {
      &AngularMotion::Constant(delta) => {
        use std::ops::Mul;
        rot.quaternion = Quat::from_rotation_z(delta).mul(rot.quaternion).into();
      }
    }
  }
}
