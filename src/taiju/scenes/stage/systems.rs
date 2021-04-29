use bevy::prelude::*;
use crate::taiju::scenes::stage::components::*;
use crate::taiju::system::UserInput;

pub fn move_sora(time: Res<Time>, input: Res<UserInput>, mut query: Query<(&mut Position), With<Sora>>) {
  for (mut pos) in query.iter_mut() {
    let pos: &mut Position = &mut pos;
    *pos.x += (*input).x * (*time).delta_seconds() * 500.0;
    *pos.y += (*input).y * (*time).delta_seconds() * 500.0;
  }
}

pub fn transform(time: Res<Time>, mut query: Query<(&Position, &mut Transform), With<Witch>>) {
  for (pos, mut trans) in query.iter_mut() {
    let pos: &Position = &pos;
    let trans: &mut Transform = &mut trans;
    trans.translation.x = *pos.x;
    trans.translation.y = *pos.y;
  }
}
