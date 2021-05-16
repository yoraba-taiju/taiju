use std::collections::HashMap;
use bevy::prelude::*;
use bevy::asset::HandleId;
use crate::chapter::scenario::EnemyDescription;
use crate::donut::ClockRef;
use super::lifecycles::Spawned;

pub struct Enemy;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum EnemyKind {
  Enemy01,
}

#[derive(Default)]
pub struct EnemyServer {
  pub sprites: HashMap<EnemyKind, SpriteBundle>,
}

impl EnemyServer {
  pub fn spawn(
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
  pub fn get_asset_handles(&self) -> Vec<HandleId> {
    let mut handles: Vec<HandleId> = Vec::new();
    for (_, it) in &self.sprites {
      handles.push(it.material.id);
      handles.push(it.mesh.id);
    }
    handles
  }
  pub fn spawn_enemy(
    &self,
    desc: &EnemyDescription,
    clock: &Res<ClockRef>,
    commands: &mut Commands,
  ) {
    let mut c = commands.spawn();
    c.insert(Enemy);
    c.insert(Spawned::new(&clock));
    match desc.enemy {
      EnemyKind::Enemy01 => {
        let mut sprite = self.sprites[&EnemyKind::Enemy01].clone();
        sprite.transform.translation.x = desc.position.x;
        sprite.transform.translation.y = desc.position.y;
        c.insert_bundle(sprite);
      }
    };
    c.insert(clock.make(desc.position));
    match desc.attack {
      // TODO
    }
  }
}
