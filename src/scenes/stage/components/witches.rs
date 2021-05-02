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

pub fn move_sora(input: Res<UserInput>, mut query: Query<(&mut Position), With<Sora>>) {
  let pos: &mut Position = &mut (query.single_mut().unwrap());
  *pos.x += (*input).x * 500.0 / 60.0;
  *pos.y += (*input).y * 500.0 / 60.0;
}
