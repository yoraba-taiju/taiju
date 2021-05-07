use crate::chapter::prelude::*;
use bevy::asset::{AssetLoader, LoadContext, LoadedAsset};

use anyhow::Result;
use bevy::utils::BoxedFuture;

#[derive(Default)]
pub struct ScenarioLoader;

impl AssetLoader for ScenarioLoader {
  fn load<'a>(
    &'a self,
    bytes: &'a [u8],
    load_context: &'a mut LoadContext,
  ) -> BoxedFuture<'a, Result<()>> {
    Box::pin(async move {
      let scenario = Scenario::try_from_bytes(&bytes)?;
      load_context.set_default_asset(LoadedAsset::new(scenario));
      Ok(())
    })
  }

  fn extensions(&self) -> &[&str] {
    &["ron"]
  }
}
