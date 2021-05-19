// https://stackoverflow.com/questions/57756927/rust-modules-confusion-when-there-is-main-rs-and-lib-rs

pub mod donut;
pub mod loading;
pub mod chapter;
pub mod prelude;
pub mod bevy_extention;

/******************************************************************************
 ** App Construction
 ******************************************************************************/

use crate::prelude::*;
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
  LoadingChapter,
  InChapter,
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
pub enum ChapterSystemLabel {
  PrepareFrame,
  UpdateStates,
  RenderFrame,
  UpdateScenario,
  EndFrame,
}

pub fn build() -> bevy::prelude::App {

  let mut builder = App::build();
  builder
    .insert_resource(WindowDescriptor {
      title: "Yoraba Taiju".to_string(),
      width: 1920.,
      height: 1080.,
      vsync: true,
      ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)));

  // Define Our States
  builder
    .add_plugin(crate::bevy_extention::TaijuPlugin)
    .insert_resource(UserInput::default());
  
  { // Prepare loading
    builder.add_state(AppState::LoadingChapter);
  }

  { // Loading Scene
    builder
      // Enter
      .add_system_set(
        SystemSet::on_enter(AppState::LoadingChapter)
          .with_system(loading::chapter::on_enter.system())
      )
      // Exit
      .add_system_set(
        SystemSet::on_exit(AppState::LoadingChapter)
          .with_system(loading::chapter::on_exit.system())
      )
      // Update
      .add_system_set(
        SystemSet::on_update(AppState::LoadingChapter)
          .with_system(loading::chapter::update.system())
      );
  }

  { // Main Game Stage
    // https://github.com/bevyengine/bevy/blob/38feddb87850424df3a0b08bae8dc32c57004798/examples/ecs/system_sets.rs
    use ChapterSystemLabel::*;
    use chapter::system as sys;
    builder
      // Enter
      .add_system_set(SystemSet::on_enter(AppState::InChapter)
        .with_system(chapter::on_enter.system())
      )
      // Exit
      .add_system_set(SystemSet::on_exit(AppState::InChapter)
        .with_system(chapter::on_exit.system())
      );

    // Updates
    builder
      .add_system_set(SystemSet::on_update(AppState::InChapter)
        .label(PrepareFrame)
        .with_system(sys::states::user_input::update.system())
        .with_system(sys::components::geometry::vanish_entity.system())
        .with_system(sys::components::lifetime::remove_future_entity.system())
        .with_system(sys::components::lifetime::restore_or_remove_vanished_entity.system())
      )
      .add_system_set(SystemSet::on_update(AppState::InChapter)
        .label(UpdateStates)
        .after(PrepareFrame)
        .with_system(sys::components::enemy::update.system())
        .with_system(sys::components::enemy_bullet::update.system())
        .with_system(sys::components::witch::sora::update.system())
      )
      .add_system_set(SystemSet::on_update(AppState::InChapter)
        .label(RenderFrame)
        .after(UpdateStates)
        .with_system(sys::components::geometry::copy_position.system())
        .with_system(sys::components::geometry::copy_rotation.system())
        .with_system(sys::components::make_visible.system())
        .with_system(sys::components::make_invisible.system())
      )
      .add_system_set(SystemSet::on_update(AppState::InChapter)
        .label(UpdateScenario)
        .after(RenderFrame)
        .with_system(sys::states::scenario_reader::update.system())
      )
      .add_system_set(SystemSet::on_update(AppState::InChapter)
        .label(EndFrame)
        .after(UpdateScenario)
        .with_system(sys::states::clock::update.system())
      );
  }
  builder.app
}
