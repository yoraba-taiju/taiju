use bevy::prelude::*;
use crate::scenes::stage::prelude::*;

pub struct Witch {
  pub health: Value<u16>,
  pub spell: Value<u16>,
}

pub struct Sora {

}

impl Sora {
  pub fn spawn(commands: &mut Commands) {

  }
  pub fn update(input: Res<UserInput>, mut query: Query<(&mut Value<Position>), With<Sora>>) {
    let pos: &mut Value<Position> = &mut (query.single_mut().unwrap());
    pos.advance(&Motion::Constant(input.x.clone() * 500.0 / 60.0, input.y.clone() * 500.0 / 60.0));
  }
}

pub struct Chitose {

}

pub struct Momiji {

}

pub struct Kaede {

}
