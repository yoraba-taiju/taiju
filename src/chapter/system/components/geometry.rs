use crate::prelude::*;

pub fn copy_position(mut query: Query<(&Value<Position>, &mut Transform), Without<Vanished>>) {
  for (pos, mut trans) in query.iter_mut() {
    let pos: &Value<Position> = &pos;
    let trans: &mut Transform = &mut trans;
    trans.translation.x = pos.x;
    trans.translation.y = pos.y;
  }
}

pub fn copy_rotation(mut query: Query<(&Value<Rotation>, &mut Transform), Without<Vanished>>) {
  for (rot, mut trans) in query.iter_mut() {
    let rot: &Value<Rotation> = &rot;
    let trans: &mut Transform = &mut trans;
    trans.rotation = rot.quaternion;
  }
}

pub fn vanish_entity(
  mut commands: Commands,
  clock: Res<ClockRef>,
  mut query: Query<(Entity, &Value<Position>), (Without<Vanished>, Without<Witch>)>
) {
  if clock.is_inspected() {
    return;
  }
  for (entity, pos) in query.iter_mut() {
    let entity: Entity = entity;
    let pos: &Value<Position> = &pos;
    let x = pos.x.clone();
    let y = pos.y.clone();
    if x < (-30.0-(1920.0/2.0)) ||
      (30.0+(1920.0/2.0)) < x ||
      y < (-30.0-(1080.0/2.0)) ||
      (30.0+(1080.0/2.0)) < y {
      commands.entity(entity).insert(Vanished::new(&clock));
    }
  }
}

