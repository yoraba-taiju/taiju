use crate::prelude::*;

pub fn update(
  clock: Res<ClockRef>,
   mut query: Query<(&mut Value<Position>, &Spawned), (With<SoraBullet>, Without<Vanished>)>
) {
  if clock.is_inspected() {
    return;
  }
  for (mut pos, Spawned{at}) in query.iter_mut() {
    pos.x += 1920.0 / 60.0;
    println!("{}", at);
  }
}
