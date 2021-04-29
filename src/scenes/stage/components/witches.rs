use bevy::prelude::*;
use crate::scenes::stage::prelude::*;

pub struct Witch {
  pub health: Value<u16>,
  pub spell: Value<u16>,
}

pub struct Sora {

}

pub struct Chitose {

}

pub struct Momiji {

}

pub struct Kaede {

}

pub fn move_sora(time: Res<Time>, input: Res<UserInput>, mut query: Query<(&mut Position), With<Sora>>) {
  for (mut pos) in query.iter_mut() {
    let pos: &mut Position = &mut pos;
    *pos.x += (*input).x * 500.0 / 60.0;
    *pos.y += (*input).y * 500.0 / 60.0;
  }
}
