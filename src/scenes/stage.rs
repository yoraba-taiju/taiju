use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod prelude;

use prelude::*;

pub struct StageScene {
}

impl StageScene {
  pub fn new() -> Self {
    Self {
    }
  }
}

pub fn setup(
  mut commands: Commands,
  clock: Res<ClockRef>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  asset_server: Res<AssetServer>,
) {
  commands.insert_resource(ScenarioDirector::load(asset_server));
  // witches
  commands.spawn()
    .insert_bundle(SpriteBundle {
      material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
      transform: Transform::from_xyz(0.0, 0.0, 0.0),
      sprite: Sprite::new(Vec2::new(10.0, 10.0)),
      ..Default::default()
    })
    .insert(Sora {
    })
    .insert(Witch {
      health: clock.value(100),
      spell: clock.value(100),
    })
    .insert(Position::new(&clock, -100.0, 0.0, 10.0, 10.0))
  ;

  // cameras
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
  commands.spawn_bundle(UiCameraBundle::default());
}