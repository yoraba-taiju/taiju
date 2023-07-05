use crate::prelude::*;

pub fn remove_future_entity (
  mut commands: Commands,
  clock: Res<ClockRef>,
  query: Query<(Entity, &Spawned)>,
) {
  if !clock.is_inspected() {
    return;
  }
  let current = clock.ticks_to_read();
  println!("remove: {}", current);
  for (entity, spawned) in query.iter() {
    if current <= spawned.at {
      commands.entity(entity).despawn_recursive();
    }
  }
}

pub fn restore_or_remove_vanished_entity (
  mut commands: Commands,
  clock: Res<ClockRef>,
  query: Query<(Entity, &Vanished)>,
) {
  if clock.is_inspected() {
    return;
  }
  let current = clock.ticks_to_read();
  for (entity, vanished) in query.iter() {
    if current < vanished.at {
      commands.entity(entity).remove::<Vanished>();
    } else if &vanished.at + (RECORDED_FRAMES as u32) < current {
      commands.entity(entity).despawn_recursive();
    }
  }
}

