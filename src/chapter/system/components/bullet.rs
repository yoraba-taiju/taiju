use lazy_static::__Deref;

use crate::prelude::*;

pub fn update(
  clock: Res<ClockRef>,
  mut bullet_query: Query<(Entity, &mut Visible, &Bullet, &mut Value<Position>), Without<Sora>>,
  sora_query: Query<&Value<Position>, With<Sora>>,
) {
  if clock.is_inspected() {
    return;
  }
  let sora_position: Position = 
    if let Ok(pos) = sora_query.single() {
      pos.deref().clone()
    } else {
      Position::new(0.0, 0.0)
    };
  for (
    _entity,
    mut visible,
    Bullet,
    mut position) in bullet_query.iter_mut()
  {
    visible.is_visible = true;
  }
}
