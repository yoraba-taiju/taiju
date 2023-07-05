pub mod witch;
pub mod enemy;
pub mod enemy_bullet;
pub mod geometry;
pub mod lifetime;

use crate::prelude::*;

pub fn make_visible(
  mut commands: Commands,
  mut query: Query<(Entity, &mut Visible), With<MakeVisible>>,
) {
  for (entity, mut visible) in query.iter_mut() {
    visible.is_visible = true;
    commands.entity(entity).remove::<MakeVisible>();
  }
}

pub fn make_invisible(
  mut commands: Commands,
  mut query: Query<(Entity, &mut Visible), With<MakeInvisible>>,
) {
  for (entity, mut visible) in query.iter_mut() {
    visible.is_visible = false;
    commands.entity(entity).remove::<MakeInvisible>();
  }
}
