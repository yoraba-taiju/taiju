use bevy::prelude::*;
use crate::taiju::scenes::stage::components::*;

pub fn transform(time: Res<Time>, mut query: Query<(&Position, &mut Transform), With<Witch>>) {
  for (pos, mut trans) in query.iter_mut() {
    let pos: &Position = &pos;
    let trans: &mut Transform = &mut trans;
  }
}