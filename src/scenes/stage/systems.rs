use bevy::prelude::*;

pub mod scenario;

use crate::donut::{ClockRef, RECORDED_FRAMES};
use crate::scenes::stage::components::*;
use crate::system::UserInput;

pub fn move_sora(time: Res<Time>, input: Res<UserInput>, mut query: Query<(&mut Position), With<Sora>>) {
  for (mut pos) in query.iter_mut() {
    let pos: &mut Position = &mut pos;
    *pos.x += (*input).x * 500.0 / 60.0;
    *pos.y += (*input).y * 500.0 / 60.0;
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

#[derive(Debug)]
pub struct Vanished {
  pub vanished_at: u32,
}

pub fn handle_vanished_entities(
  mut commands: Commands,
  clock: Res<ClockRef>,
  mut query: Query<(Entity, &Vanished)>,
) {
  let current = clock.current_tick();
  for (entity, vanished) in query.iter() {
    let entity: Entity = entity;
    let vanished: &Vanished = vanished;
    if vanished.vanished_at + (RECORDED_FRAMES as u32) < current {
      commands.entity(entity).despawn_recursive();
    }else if current < vanished.vanished_at {
      commands.entity(entity).remove::<Vanished>();
    }
  }
}
