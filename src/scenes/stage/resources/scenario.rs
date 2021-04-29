use crate::scenes::stage::prelude::*;
use std::f32::consts::PI;

enum EventType {

}

struct Event {

}

pub struct ScenarioDirector {

}

impl ScenarioDirector {
  pub fn load(asset_server: Res<AssetServer>) -> Self {
    Self{
    }
  }
}

pub fn progress_scenario(
  mut commands: Commands,
  mut materials: ResMut<Assets<ColorMaterial>>,
  sora_query: Query<(Entity, &Position), With<Sora>>,
  clock: Res<ClockRef>,
  user_input: Res<UserInput>,
  mut director: ResMut<ScenarioDirector>,
) {
  if user_input.clock_direction <= 0 {
    return;
  }

  if clock.current_tick() % 60 == 1 {
    for i in -5..=5 {
      let angle = ((i+18) as f32) * 2.0 * PI / 36.0;
      commands.spawn()
        .insert(Lifetime::new(&clock))
        .insert(Bullet{})
        .insert(Position::new(&clock,400.0, 0.0, 10.0, 10.0))
        .insert(Motion::Constant(angle.cos() * 3.0, angle.sin() * 3.0))
        .insert_bundle(SpriteBundle {
          material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
          transform: Transform::from_xyz(0.0, 0.0, 0.0),
          sprite: Sprite::new(Vec2::new(10.0, 10.0)),
          ..Default::default()
        })
      ;
    }
  }
  if clock.current_tick() % 7 == 2 {
    let sora: (Entity, &Position) = sora_query.iter().next().unwrap();
    let from = (400.0f32,0.0f32);
    let mut dir = (*sora.1.x - from.0, *sora.1.y - from.1);
    let sum = (dir.0.powf(2.0) + dir.1.powf(2.0)).sqrt();
    dir.0 /= sum;
    dir.1 /= sum;
    commands.spawn()
      .insert(Lifetime::new(&clock))
      .insert(Bullet{})
      .insert(Position::new(&clock,400.0, 0.0, 10.0, 10.0))
      .insert(Motion::Constant(dir.0 * 10.0, dir.1 * 10.0))
      .insert_bundle(SpriteBundle {
        material: materials.add(Color::rgb(0.0, 0.5, 0.5).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        sprite: Sprite::new(Vec2::new(10.0, 10.0)),
        ..Default::default()
      })
    ;
  }
}