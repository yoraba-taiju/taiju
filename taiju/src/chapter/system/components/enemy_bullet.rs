use crate::prelude::*;

pub fn update(
  clock: Res<ClockRef>,
  mut bullet_query: Query<(Entity, &mut Visible, &EnemyBullet, &mut Value<Position>), (Without<Sora>, Without<Vanished>)>,
  sora_query: Query<&Value<Position>, (With<Sora>, Without<Vanished>)>,
) {
  if clock.is_inspected() {
    return;
  }
  let sora_position: Position = 
    if let Ok(pos) = sora_query.single() {
      **pos
    } else {
      Position::new(0.0, 0.0)
    };
  for (
    _entity,
    mut visible,
    EnemyBullet(bullet_kind, attack_kind),
    mut position
  ) in bullet_query.iter_mut()
  {
    visible.is_visible = true;
  }
}
