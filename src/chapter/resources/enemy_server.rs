use std::collections::HashMap;
use crate::prelude::*;
use crate::chapter::scenario::EnemyDescription;

pub struct EnemyServer {
  pub sprites: HashMap<EnemyKind, SpriteBundle>,
}

impl EnemyServer {
  pub fn from_loader(loader: &mut Loader) -> Self {
    let mut s = Self{
      sprites: Default::default(),
    };
    let mut load = |enemy_kind: EnemyKind, path: &str| {
      s.sprites.insert(enemy_kind, loader.load_sprite(path));
    };
    load(EnemyKind::Enemy01, "sprites/bullets/blue_small.png");
    s
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
    c.insert(clock.make(desc.position.clone()));
    match desc.attack {
      // TODO
    }
  }
}
