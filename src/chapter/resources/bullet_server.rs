use std::collections::HashMap;
use crate::prelude::*;

pub struct BulletServer {
  pub sprites: HashMap<BulletKind, SpriteBundle>,
}

impl BulletServer {
  pub fn from_loader(loader: &mut Loader) -> Self {
    let mut s = Self{
      sprites: Default::default(),
    };
    let mut load = |bullet_kind: BulletKind, path: &str| {
      let mut spr = loader.load_sprite(path);
      spr.visible.is_visible = false;
      s.sprites.insert(bullet_kind, spr);
    };
    load(BulletKind::BlueSmall, "sprites/bullets/blue_small.png");
    load(BulletKind::RedSmall, "sprites/bullets/red_small.png");
    load(BulletKind::BlueNeedle, "sprites/bullets/blue_needle.png");
    load(BulletKind::RedNeedle, "sprites/bullets/red_needle.png");
    s
  }

  pub fn spawn(
    commands: &mut Commands,
    bullet_kind: BulletKind,
  ) {
    todo!();
  }
}