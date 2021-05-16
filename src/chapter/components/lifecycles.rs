use crate::prelude::*;
use crate::donut::RECORDED_FRAMES;

#[derive(Debug)]
pub struct Spawned {
  pub at: u32,
}

impl Spawned {
  pub(crate) fn new(clock: &ClockRef) ->Self {
    Self {
      at: clock.current_time().ticks,
    }
  }
}

#[derive(Debug)]
pub struct Vanished {
  pub at: u32,
}

impl Vanished {
  pub(crate) fn new(clock: &ClockRef) ->Self {
    Self {
      at: clock.current_time().ticks,
    }
  }
}

pub fn handle_lifetime(
  mut commands: Commands,
  clock: Res<ClockRef>,
  user_input: Res<UserInput>,
  mut s_query: Query<(Entity, &mut Spawned)>,
  mut v_query: Query<(Entity, &mut Vanished)>,
) {
  let current = clock.ticks_to_read();
  if clock.is_inspected() {
    for (entity, mut spawned) in s_query.iter_mut() {
      let entity: Entity = entity;
      let spawned: &mut Spawned = &mut spawned;
      if current <= spawned.at {
        commands.entity(entity).despawn_recursive();
      }
    }
  } else {
    for (entity, mut vanished) in v_query.iter_mut() {
      let entity: Entity = entity;
      let vanished: &mut Vanished = &mut vanished;
      if current < vanished.at {
        commands.entity(entity).remove::<Vanished>();
      } else if &vanished.at + (RECORDED_FRAMES as u32) < current {
        commands.entity(entity).despawn_recursive();
      }
    }
  }
}

