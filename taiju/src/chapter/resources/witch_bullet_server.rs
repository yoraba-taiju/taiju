use crate::prelude::*;

pub struct WitchBulletServer {
  sora_bullet_sprite: SpriteBundle,
}

impl WitchBulletServer {
  pub fn from_loader(loader: &mut Loader) -> Self {
    let mut s = Self{
      sora_bullet_sprite: Default::default(),
    };
    let mut load = |spr: &mut SpriteBundle, path: &str| {
      *spr = loader.load_sprite(path);
      spr.visible.is_visible = false;
    };
    load(&mut s.sora_bullet_sprite, "sprites/bullets/sora.png");
    s
  }

  pub fn spawn_sora_bullet(
    &self,
    clock: &ClockRef,
    commands: &mut Commands,
    at: Position,
  ) {
    commands
      .spawn_bundle(self.sora_bullet_sprite.clone())
      .insert_bundle((
        SoraBullet,
        clock.make(at),
        Spawned::new(clock),
      ))
      .insert(MakeVisible);
  }
}