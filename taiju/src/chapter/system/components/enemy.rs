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
      Position::ZERO
    };
  for (
    _entity,
    Spawned { at: spawned_at },
    Enemy(_enemy_kind, attack_kind),
    mut position,
    mut velocity
  ) in enemy_query.iter_mut()
  {
    let spawned_from = clock.current_time().ticks - spawned_at;
    match attack_kind.clone() {
      EnemyAttackKind::JustFly(attack_velocity) => {
        **velocity = attack_velocity;
      }
      EnemyAttackKind::Kamikaze {speed, max_angle} => {
        let current_speed = velocity.to_vec2();
        if spawned_from < 60 {
          let vec_to_sora = (sora_position - **position).normalize();

          let max_angle = max_angle * std::f32::consts::PI / 180.0;
          let max_vec_l = Mat2::from_angle(max_angle).mul_vec2(current_speed).normalize();
          let max_vec_r = Mat2::from_angle(-max_angle).mul_vec2(current_speed).normalize();

          let attack_velocity: Vec2 = 
            if vec_to_sora.dot(current_speed) <= max_vec_l.dot(current_speed) {
              vec_to_sora
            } else if max_vec_l.dot(vec_to_sora) <= max_vec_r.dot(vec_to_sora) {
              max_vec_l
            } else {
              max_vec_r
            } * speed;
          **velocity = Velocity::from_vec2(&attack_velocity);
        } else {
          if spawned_from == 60 {
            let attack_velocity = current_speed * 3.0;
            **velocity = Velocity::from_vec2(&attack_velocity);
          }
        }
      }
    }
    position.x += velocity.dx;
    position.y += velocity.dy;
  }
}
