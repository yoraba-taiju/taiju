use std::collections::HashMap;
use crate::prelude::*;
#[derive(Default)]
pub struct BulletServer {
  pub sprites: HashMap<BulletKind, SpriteBundle>,
}

impl BulletServer {
  pub fn get_asset_handles(&self) -> Vec<HandleId> {
    let mut handles = Vec::new();
    self.sprites.values().for_each(|it| {
      handles.push(it.material.id);
      handles.push(it.mesh.id);
    });
    handles
  }
}