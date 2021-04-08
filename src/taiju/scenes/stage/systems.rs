use bevy::prelude::*;
use crate::taiju::scenes::stage::components::*;

pub fn enumrate(mut query: Query<(&Position, &mut Transform), With<Witch>>) {
  for (pos, mut trans) in query.iter_mut() {
    let trans: &mut Transform = &mut trans;
    //println!("pos: {:?}", pos.x);
    trans.translation.x += 0.1;
    trans.translation.y += 0.1;
  }
}