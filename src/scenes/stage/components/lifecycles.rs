use crate::scenes::stage::prelude::*;
use crate::donut::RECORDED_FRAMES;

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

