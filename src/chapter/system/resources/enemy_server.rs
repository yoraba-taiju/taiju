use std::collections::HashMap;
use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use bevy::asset::HandleId;
use crate::chapter::scenario::EnemyDescription;

pub fn setup(
  commands: &mut Commands,
  asset_server: &Res<AssetServer>,
  color_materials: &mut ResMut<Assets<ColorMaterial>>,
  texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
  let mut s = EnemyServer::default();
  { // Enemy 01
    let texture = asset_server.load::<Texture, _>("sprites/bullets/blue_small.png");
    let color_material = color_materials.add(texture.into());
    s.sprites.insert(EnemyKind::Enemy01, SpriteBundle {
      material: color_material,
      ..Default::default()
    });
  }
  commands.insert_resource(s);
}
