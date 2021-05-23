use std::collections::HashMap;
use crate::prelude::*;

pub struct WitchServer {
  pub sprites: HashMap<WitchKind, SpriteBundle>,
}

impl WitchServer {
  pub fn from_loader(loader: &mut Loader) -> Self {
    let mut s = Self{
      sprites: Default::default(),
    };
    let mut load = |witch_kind: WitchKind, path: &str| {
      let mut spr = loader.load_sprite(path);
      spr.visible.is_visible = false;
      s.sprites.insert(witch_kind, spr);
    };
    load(WitchKind::Sora, "sprites/witches/sora.png");
    load(WitchKind::Chitose, "sprites/witches/sora.png");
    load(WitchKind::Momiji, "sprites/witches/sora.png");
    load(WitchKind::Kaede, "sprites/witches/sora.png");
    s
  }

  pub fn spawn(
    &self,
    clock: &ClockRef,
    commands: &mut Commands,
    witch_kind: WitchKind,
    position: Position,
  ) {
    let mut c = commands.spawn();
    c.insert(Spawned::new(clock));
    c.insert(Witch {
      health: clock.make(100),
      spell: clock.make(100),
    });
    c.insert(clock.make(position));
    c.insert(MakeVisible);
    c.insert_bundle(self.sprites[&witch_kind].clone());

    match witch_kind {
      WitchKind::Sora => {
        c.insert(Sora);
      }
      WitchKind::Chitose => {
        c.insert(Chitose);
      }
      WitchKind::Momiji => {
        c.insert(Momiji);
      }
      WitchKind::Kaede => {
        c.insert(Kaede);
      }
    }
  }
}