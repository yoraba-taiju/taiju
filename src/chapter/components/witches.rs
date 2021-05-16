use bevy::prelude::*;
use crate::prelude::*;

pub struct Witch {
  pub health: Value<u16>,
  pub spell: Value<u16>,
}

pub struct Sora;

impl Sora {
  pub fn spawn(
    commands: &mut Commands,
    clock: &Res<ClockRef>,
    asset_server: &Res<AssetServer>,
    color_materials: &mut ResMut<Assets<ColorMaterial>>
  ) {
    let texture_handle = asset_server.load("sprites/witches/sora.png");
    commands.spawn()
      .insert(Sora)
      .insert(Witch {
        health: clock.make(100),
        spell: clock.make(100),
      })
      .insert(clock.make(Position::new(-600.0, 0.0)))
      .insert_bundle(SpriteBundle {
        material: color_materials.add(texture_handle.into()),
        transform: Transform::from_scale(Vec3::new(0.75, 0.75, 0.75)),
        ..Default::default()
      });
  }
  pub fn update(clock: Res<ClockRef>, input: Res<UserInput>, mut query: Query<&mut Value<Position>, With<Sora>>) {
    if clock.is_inspected() {
      return;
    }
    let pos: &mut Value<Position> = &mut (query.single_mut().unwrap());
    pos.x += input.x * 500.0 / 60.0;
    pos.y += input.y * 500.0 / 60.0;
  }
}

pub struct Chitose;

pub struct Momiji;

pub struct Kaede;
