use bevy::prelude::*;
use crate::scenes::stage::prelude::*;

pub struct Witch {
  pub health: Value<u16>,
  pub spell: Value<u16>,
}

pub struct Sora {

}

impl Sora {
  pub fn spawn(
    commands: &mut Commands,
    clock: &Res<ClockRef>,
    asset_server: &Res<AssetServer>,
    color_materials: &mut ResMut<Assets<ColorMaterial>>
  ) {
    let texture_handle = asset_server.load("sprites/witches/sora.png");
    commands.spawn()
      .insert(Sora {
      })
      .insert(Witch {
        health: clock.make(100),
        spell: clock.make(100),
      })
      .insert(clock.make(Position {
        x: -700.0,
        y: 0.0,
      }))
      .insert_bundle(SpriteBundle {
        material: color_materials.add(texture_handle.into()),
        transform: Transform::from_scale(Vec3::new(0.75, 0.75, 0.75)),
        ..Default::default()
      })
    ;
  }
  pub fn update(input: Res<UserInput>, mut query: Query<(&mut Value<Position>), With<Sora>>) {
    let pos: &mut Value<Position> = &mut (query.single_mut().unwrap());
    pos.apply(&Motion::Constant(input.x.clone() * 500.0 / 60.0, input.y.clone() * 500.0 / 60.0));
  }
}

pub struct Chitose {

}

pub struct Momiji {

}

pub struct Kaede {

}
