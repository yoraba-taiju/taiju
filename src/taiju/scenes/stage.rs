use bevy::prelude::*;

pub mod components;
pub mod systems;

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
  mut materials: ResMut<Assets<ColorMaterial>>,
  asset_server: Res<AssetServer>,
) {
  use components::*;
  // cameras
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
  commands.spawn_bundle(UiCameraBundle::default());
  // witches
  commands.spawn()
    .insert_bundle(SpriteBundle {
      material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
      transform: Transform::from_xyz(0.0, -100.0, 0.0),
      sprite: Sprite::new(Vec2::new(100.0, 100.0)),
      ..Default::default()
    })
    .insert(Sora {
    })
    .insert(Witch {
      health: 100,
      spell: 100,
    })
    .insert(Position {
      x: 0.0,
      y: 0.,
      w: 10.,
      h: 10.,
    });
}