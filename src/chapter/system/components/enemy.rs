use lazy_static::__Deref;

use crate::prelude::*;
use bevy::math::Mat2;

pub fn update(
  clock: Res<ClockRef>,
  mut enemy_query: Query<(Entity, &Enemy, &mut Value<Position>, &mut Value<Velocity>), (Without<Sora>, Without<Vanished>)>,
  sora_query: Query<&Value<Position>, (With<Sora>, Without<Vanished>)>,
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
    Enemy(enemy_kind, attack_kind),
    mut position,
    mut velocity) in enemy_query.iter_mut()
  {
    match attack_kind.clone() {
      EnemyAttackKind::JustFly(velocity) => {
        position.x += velocity.dx;
        position.y += velocity.dy;
      }
      EnemyAttackKind::Kamikaze {speed, max_angle} => {
        let to_sora = Vec2::new(sora_position.x - position.x, sora_position.y - position.y).normalize();
        let current_speed = Vec2::new(velocity.dx, velocity.dy);
        let max_angle_plus = Mat2::from_angle(max_angle).mul_vec2(current_speed);
        let max_angle_minus = Mat2::from_angle(-max_angle).mul_vec2(current_speed);

        let velocity: Vec2 = 
          if to_sora.dot(current_speed) <= max_angle_plus.dot(current_speed) {
            to_sora
          } else if max_angle_plus.dot(to_sora) <= max_angle_minus.dot(to_sora) {
            max_angle_plus
          } else {
            max_angle_minus
          } * speed;
        position.x += velocity.x;
        position.y += velocity.y;
      }
    }
  }
}
