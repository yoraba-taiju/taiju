use crate::scenes::stage::prelude::*;

#[derive(Debug)]
pub struct Position {
  pub x: Value<f32>,
  pub y: Value<f32>,
  pub w: Value<f32>,
  pub h: Value<f32>,
}

pub struct Motion {
  pub speed: (f32, f32),
}

pub fn copy_to_transform(time: Res<Time>, mut query: Query<(&Position, &mut Transform), With<Witch>>) {
  for (pos, mut trans) in query.iter_mut() {
    let pos: &Position = &pos;
    let trans: &mut Transform = &mut trans;
    trans.translation.x = *pos.x;
    trans.translation.y = *pos.y;
  }
}
