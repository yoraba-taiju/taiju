use bevy::prelude::*;
use crate::scenes::stage::prelude::*;
use std::collections::HashMap;

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
  pub fn spawn_enemy(
    &self,
    desc: &EnemyDescription,
    clock: &Res<ClockRef>,
    commands: &mut Commands,
  ) {
    let mut c = commands.spawn();
    match desc.body {
      EnemyBody::Enemy01 => {
        c.insert_bundle(self.sprites[&EnemyBody::Enemy01].clone());
      }
    };
    c.insert(clock.make(desc.position.clone()));
    if let Some(motion) = desc.motion {
      c.insert(motion.clone());
    }
    if desc.rotation.is_some() || desc.angular_motion.is_some() {
      c.insert(clock.make(Rotation {
        quaternion: Quat::from_rotation_z(desc.rotation.unwrap_or_default()),
      }));
      c.insert(desc.angular_motion.unwrap_or_default());
    }
  }
}
