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
      let mut spr = loader.load_sprite(path);
      spr.visible.is_visible = false;
      s.sprites.insert(enemy_kind, spr);
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
    c.insert_bundle(self.sprites[&desc.enemy].clone());
    c.insert(Enemy(desc.enemy.clone()));
    c.insert(EnemyAttack(desc.attack.clone()));
    c.insert(Spawned::new(&clock));
    c.insert(desc.position.clone());
    c.insert(clock.make(desc.position.clone()));
  }
}
