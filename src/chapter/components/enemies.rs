use bevy::prelude::*;
use crate::chapter::prelude::*;
use std::collections::HashMap;
use bevy::asset::HandleId;

pub struct Enemy {

}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum EnemyBody {
  Enemy01,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EnemyDescription {
  pub body: EnemyBody,
  pub position: Position,
  pub motion: Option<Motion>,
  pub rotation: Option<f32>,
  pub angular_motion: Option<AngularMotion>,
}

#[derive(Default)]
pub struct EnemyServer {
  pub sprites: HashMap<EnemyBody, SpriteBundle>,
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
      s.sprites.insert(EnemyBody::Enemy01, SpriteBundle {
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
    c.insert(Enemy{});
    c.insert(Spawned::new(&clock));
    match desc.body {
      EnemyBody::Enemy01 => {
        let mut sprite = self.sprites[&EnemyBody::Enemy01].clone();
        sprite.transform.translation.x = desc.position.x;
        sprite.transform.translation.y = desc.position.y;
        c.insert_bundle(sprite);
      }
    };
    c.insert(clock.make(desc.position));
    if let Some(motion) = desc.motion {
      c.insert(motion);
    }
    if desc.rotation.is_some() || desc.angular_motion.is_some() {
      c.insert(clock.make(Rotation {
        quaternion: Quat::from_rotation_z(desc.rotation.unwrap_or_default()),
      }));
      c.insert(desc.angular_motion.unwrap_or_default());
    }
  }
}
