use crate::scenes::stage::prelude::*;
use crate::donut::RECORDED_FRAMES;

#[derive(Debug)]
pub struct Lifetime {
  pub spawned_at: u32,
  pub vanished_at: Option<u32>,
}

impl Lifetime {
  pub(crate) fn new(clock: &ClockRef) ->Self {
    Self {
      spawned_at: clock.current_tick(),
      vanished_at: None,
    }
  }
}

pub fn handle_lifetime(
  mut commands: Commands,
  clock: Res<ClockRef>,
  user_input: Res<UserInput>,
  mut query: Query<(Entity, &mut Lifetime)>,
) {
  let current = clock.current_tick();
  if user_input.clock_direction <= 0 {
    for (entity, mut lifetime) in query.iter_mut() {
      let entity: Entity = entity;
      let lifetime: &mut Lifetime = &mut lifetime;
      if current <= lifetime.spawned_at {
        commands.entity(entity).despawn_recursive();
      }
    }
  } else {
    for (entity, mut lifetime) in query.iter_mut() {
      let entity: Entity = entity;
      let lifetime: &mut Lifetime = &mut lifetime;
      if let Some(vanished_at) = lifetime.vanished_at {
        if vanished_at + (RECORDED_FRAMES as u32) < current {
          commands.entity(entity).despawn_recursive();
        }else if current < vanished_at {
          lifetime.vanished_at = None;
        }
      }
    }
  }
}

