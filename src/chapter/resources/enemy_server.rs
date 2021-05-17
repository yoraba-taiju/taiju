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
    clock: &Res<ClockRef>,
    commands: &mut Commands,
    enemy_kind: EnemyKind,
    attack_kind: EnemyAttackKind,
    position: Position,
  ) {
    let mut c = commands.spawn();
    c.insert(Spawned::new(&clock));
    c.insert(MakeVisible);

    c.insert(Enemy(enemy_kind));
    c.insert(EnemyAttack(attack_kind));
    c.insert_bundle(self.sprites[&enemy_kind].clone());
    c.insert(clock.make(position));
  }
}
