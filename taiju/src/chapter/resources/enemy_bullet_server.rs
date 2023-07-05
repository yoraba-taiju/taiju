use std::collections::HashMap;
use crate::prelude::*;

pub struct EnemyBulletServer {
  pub sprites: HashMap<EnemyBulletKind, SpriteBundle>,
}

impl EnemyBulletServer {
  pub fn from_loader(loader: &mut Loader) -> Self {
    let mut s = Self{
      sprites: Default::default(),
    };
    let mut load = |bullet_kind: EnemyBulletKind, path: &str| {
      let mut spr = loader.load_sprite(path);
      spr.visible.is_visible = false;
      s.sprites.insert(bullet_kind, spr);
    };
    load(EnemyBulletKind::BlueSmall, "sprites/bullets/blue_small.png");
    load(EnemyBulletKind::RedSmall, "sprites/bullets/red_small.png");
    load(EnemyBulletKind::BlueNeedle, "sprites/bullets/blue_needle.png");
    load(EnemyBulletKind::RedNeedle, "sprites/bullets/red_needle.png");
    s
  }

  pub fn spawn(
    commands: &mut Commands,
    bullet_kind: EnemyBulletKind,
  ) {
    todo!();
  }
}