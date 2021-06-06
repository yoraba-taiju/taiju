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
pub enum ChapterUpdate {
  UpdateStates,
  RenderFrame,
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
pub enum PostChapterUpdate {
  TickClock,
  PrepareNextFrame,
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, StageLabel)]
pub enum ChapterStage {
  PreUpdate,
  PostUpdate,
}

pub fn build() -> bevy::prelude::App {
  let mut builder = App::build();
  builder
    .add_plugins(DefaultPlugins)
    .insert_resource(WindowDescriptor {
      title: "Yoraba Taiju".to_string(),
      width: 1920.,
      height: 1080.,
      vsync: true,
      ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)));

  // Define Our States
  builder
    .add_plugin(crate::bevy_extention::TaijuPlugin)
    .insert_resource(UserInput::default());
  
  { // Prepare loading
    builder.add_state_to_stage(CoreStage::Update, AppState::LoadingChapter);
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
    use chapter::system as sys;
    builder.add_stage_before(CoreStage::Update, ChapterStage::PreUpdate, SystemStage::parallel());
    builder.add_stage_after(CoreStage::Update, ChapterStage::PostUpdate, SystemStage::parallel());

    // On Enter
    let on_enter_system_set = SystemSet::on_enter(AppState::InChapter)
      .with_system(chapter::on_enter.system());

    // On Exit
    let on_exit_system_set = SystemSet::on_exit(AppState::InChapter)
      .with_system(chapter::on_exit.system());

    // On Update
    let prepare_current_frame_set = SystemSet::on_update(AppState::InChapter)
        .with_system(sys::states::scenario_reader::update.system())
        .with_system(sys::states::user_input::update.system());

    let update_system_set = SystemSet::on_update(AppState::InChapter)
        .label(ChapterUpdate::UpdateStates)
          .with_system(sys::components::enemy::update.system())
          .with_system(sys::components::enemy_bullet::update.system())
          .with_system(sys::components::witch::sora::update.system())
          .with_system(sys::components::witch::sora_bullet::update.system());

    let render_system_set = SystemSet::on_update(AppState::InChapter)
      .label(ChapterUpdate::RenderFrame)
      .after(ChapterUpdate::UpdateStates)
        .with_system(sys::components::geometry::copy_position.system())
        .with_system(sys::components::geometry::copy_rotation.system())
        .with_system(sys::components::make_visible.system())
        .with_system(sys::components::make_invisible.system());

    let clock_tick_set = SystemSet::on_update(AppState::InChapter)
      .label(PostChapterUpdate::TickClock)
        .with_system(sys::states::clock::update.system());

    let prepare_next_frame_set = SystemSet::on_update(AppState::InChapter)
      .label(PostChapterUpdate::PrepareNextFrame)
      .after(PostChapterUpdate::TickClock)
        .with_system(sys::components::lifetime::remove_future_entity.system())
        .with_system(sys::components::lifetime::restore_or_remove_vanished_entity.system())
        .with_system(sys::components::geometry::vanish_entity.system());

    builder
      .add_system_set_to_stage(CoreStage::Update, on_enter_system_set)
      .add_system_set_to_stage(CoreStage::Update, on_exit_system_set)
      .add_system_set_to_stage(ChapterStage::PreUpdate, prepare_current_frame_set)
      .add_system_set_to_stage(CoreStage::Update, update_system_set)
      .add_system_set_to_stage(CoreStage::Update, render_system_set)
      .add_system_set_to_stage(ChapterStage::PostUpdate, clock_tick_set)
      .add_system_set_to_stage(ChapterStage::PostUpdate, prepare_next_frame_set)
    ;
  }
  builder.app
}
