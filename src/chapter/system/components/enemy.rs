use crate::prelude::*;
use bevy::math::Mat2;

pub fn update(
  clock: Res<ClockRef>,
  mut enemy_query: Query<(Entity, &Spawned, &Enemy, &mut Value<Position>, &mut Value<Velocity>), (Without<Sora>, Without<Vanished>)>,
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
    Spawned { at: spawned_at },
    Enemy(_enemy_kind, attack_kind),
    mut position,
    mut velocity) in enemy_query.iter_mut()
  {
    let spawned_from = clock.current_time().ticks - spawned_at;
    match attack_kind.clone() {
      EnemyAttackKind::JustFly(attack_velocity) => {
        **velocity = attack_velocity;
      }
      EnemyAttackKind::Kamikaze {speed, max_angle} => {
        let current_speed = Vec2::new(velocity.dx, velocity.dy);
        if spawned_from < 60 {
          let to_sora = Vec2::new(sora_position.x - position.x, sora_position.y - position.y).normalize();
          let max_angle = max_angle * std::f32::consts::PI / 180.0;
          let max_angle_plus = Mat2::from_angle(max_angle).mul_vec2(current_speed).normalize();
          let max_angle_minus = Mat2::from_angle(-max_angle).mul_vec2(current_speed).normalize();
          let attack_velocity: Vec2 = 
            if to_sora.dot(current_speed) <= max_angle_plus.dot(current_speed) {
              to_sora
            } else if max_angle_plus.dot(to_sora) <= max_angle_minus.dot(to_sora) {
              max_angle_plus
            } else {
              max_angle_minus
            } * speed;
          velocity.dx = attack_velocity.x;
          velocity.dy = attack_velocity.y;
        } else {
          if spawned_from == 60 {
            let attack_velocity = current_speed * 3.0;
            velocity.dx = attack_velocity.x;
            velocity.dy = attack_velocity.y;
          }
        }
      }
    }
    position.x += velocity.dx;
    position.y += velocity.dy;
  }
}
