use crate::scenes::stage::prelude::*;

#[derive(Debug)]
pub struct Position {
  pub x: Value<f32>,
  pub y: Value<f32>,
  pub w: Value<f32>,
  pub h: Value<f32>,
}

impl Position {
  pub(crate) fn new(
    clock: &ClockRef,
    x: f32, y: f32,
    w: f32, h: f32,
  ) -> Self {
    Self {
      x: clock.value(x),
      y: clock.value(y),
      w: clock.value(w),
      h: clock.value(h),
    }
  }
}

pub enum Motion {
  Constant(f32, f32),
}

pub fn move_by_motion(_clock: Res<ClockRef>, mut query: Query<(&mut Position, &Motion)>) {
  for (mut pos, motion) in query.iter_mut() {
    let pos: &mut Position = &mut pos;
    let motion: &Motion = &motion;
    use Motion::*;
    match motion {
      Constant(x, y) => {
        *pos.x += x;
        *pos.y += y;
      }
    }
  }
}

pub fn copy_to_transform(input: Res<UserInput>, mut query: Query<(&Position, &mut Transform)>) {
  for (pos, mut trans) in query.iter_mut() {
    let pos: &Position = &pos;
    let trans: &mut Transform = &mut trans;
    trans.translation.x = *pos.x;
    trans.translation.y = *pos.y;
  }
}

pub fn handle_entity_vanishes(
  mut commands: Commands,
  clock: Res<ClockRef>,
  mut query: Query<(Entity, &Position), Without<Vanished>>
) {
  for (entity, pos) in query.iter_mut() {
    let entity: Entity = entity;
    let pos: &Position = &pos;
    let x = *pos.x;
    let y = *pos.y;
    if x < (-30.0-(1920.0/2.0)) ||
      (30.0+(1920.0/2.0)) < x ||
      y < (-30.0-(1080.0/2.0)) ||
      (30.0+(1080.0/2.0)) < y {
      commands.entity(entity).insert(Vanished::new(&clock));
    }
  }
}