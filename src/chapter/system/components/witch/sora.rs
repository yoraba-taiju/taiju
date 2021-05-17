use crate::prelude::*;

pub fn update(
  clock: Res<ClockRef>,
  input: Res<UserInput>,
  mut query: Query<&mut Value<Position>, With<Sora>>
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
}
