use crate::prelude::*;

pub fn update(
  clock: Res<ClockRef>,
  mut query: Query<&mut Value<Position>, With<Enemy>>
) {
}
