use crate::prelude::*;

use bevy::asset::{Asset, Handle, HandleId};

pub struct Loader<'a> {
  asset_server: &'a Res<'a, AssetServer>,
  color_materials: &'a mut ResMut<'a, Assets<ColorMaterial>>,
  handle_ids: Vec<HandleId>,
}

impl <'a> Loader<'a>
where
{
  pub fn new(
    asset_server: &'a Res<'a, AssetServer>,
    color_materials: &'a mut ResMut<'a, Assets<ColorMaterial>>,
  ) -> Self
  where
  {
    Self {
      asset_server,
      color_materials,
      handle_ids: Default::default(),
    }
  }
  pub fn load<T: Asset>(&mut self, path: &str) -> Handle<T> {
    let handle: Handle<T> = self.asset_server.load(path);
    self.handle_ids.push(handle.id);
    handle
  }
  pub fn load_sprite(&mut self, path: &str)-> SpriteBundle {
    let handle: Handle<Texture> = self.load(path);
    let material = self.color_materials.add(handle.into());
    SpriteBundle {
      material,
      ..Default::default()
    }
  }
  pub fn build(self) -> LoadingStatus {
    LoadingStatus {
      handle_ids: self.handle_ids,
    }
  }
}

pub struct LoadingStatus {
  handle_ids: Vec<HandleId>,
}

impl LoadingStatus {
  pub fn is_loaded(&self, asset_server: &Res<AssetServer>) -> bool {
    use bevy::asset::LoadState::Loaded;
    Loaded == asset_server.get_group_load_state(self.handle_ids.clone())
  }
}
