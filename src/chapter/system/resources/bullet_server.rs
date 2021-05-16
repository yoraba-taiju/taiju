use crate::prelude::*;
use crate::chapter::states::BulletServer;

pub fn setup(
  commands: &mut Commands,
  asset_server: &Res<AssetServer>,
  color_materials: &mut ResMut<Assets<ColorMaterial>>,
  texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
  let mut s = BulletServer::default();
  { // BlueSmall
    let texture = asset_server.load::<Texture, _>("sprites/bullets/blue_small.png");
    let color_material = color_materials.add(texture.into());
    s.sprites.insert(BulletKind::BlueSmall, SpriteBundle {
      material: color_material,
      ..Default::default()
    });
  }
  { // RedSmall
    let texture = asset_server.load::<Texture, _>("sprites/bullets/red_small.png");
    let color_material = color_materials.add(texture.into());
    s.sprites.insert(BulletKind::RedSmall, SpriteBundle {
      material: color_material,
      ..Default::default()
    });
  }
  { // BlueNeedle
    let texture = asset_server.load::<Texture, _>("sprites/bullets/blue_needle.png");
    let color_material = color_materials.add(texture.into());
    s.sprites.insert(BulletKind::BlueNeedle, SpriteBundle {
      material: color_material,
      ..Default::default()
    });
  }
  { // RedNeedle
    let texture = asset_server.load::<Texture, _>("sprites/bullets/red_needle.png");
    let color_material = color_materials.add(texture.into());
    s.sprites.insert(BulletKind::RedNeedle, SpriteBundle {
      material: color_material,
      ..Default::default()
    });
  }
  commands.insert_resource(s);
}
