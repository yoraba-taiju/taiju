use lazy_static::__Deref;

use crate::prelude::*;

pub fn update(
  clock: Res<ClockRef>,
  mut enemy_query: Query<(Entity, &Enemy, &EnemyAttack, &mut Value<Position>), Without<Sora>>,
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
    Enemy(enemy_kind),
    EnemyAttack(attack_kind),
    mut position) in enemy_query.iter_mut()
  {
  }
}
