use crate::prelude::*;

pub fn update(
  mut commands: Commands,
  clock: Res<ClockRef>,
  input: Res<UserInput>,
  bullet_server: Res<WitchBulletServer>,
  mut query: Query<&mut Value<Position>, (With<Sora>, Without<Vanished>)>
) {
  if clock.is_inspected() {
    return;
  }
  let mut pos = if let Ok(pos) = query.single_mut() {
    pos
  } else {
    return;
  };
  pos.x += input.x * 500.0 / 60.0;
  pos.y += input.y * 500.0 / 60.0;
  if clock.current_time().ticks % 20 == 0 {
    bullet_server.spawn_sora_bullet(&clock, &mut commands, **pos);
  }
}
