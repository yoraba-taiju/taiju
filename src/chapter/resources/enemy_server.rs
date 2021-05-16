use std::collections::HashMap;
use crate::prelude::*;
use bevy::asset::HandleId;
use crate::chapter::scenario::EnemyDescription;

#[derive(Default)]
pub struct EnemyServer {
  pub sprites: HashMap<EnemyKind, SpriteBundle>,
}

impl EnemyServer {
  pub fn get_asset_handles(&self) -> Vec<HandleId> {
    let mut handles: Vec<HandleId> = Vec::new();
    for (_, it) in &self.sprites {
      handles.push(it.material.id);
      handles.push(it.mesh.id);
    }
    handles
  }
  pub fn spawn(
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
