use bevy::prelude::*;
use crate::stage::prelude::*;
use crate::donut::Clock;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
  Loading,
  Stage,
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
pub enum Stage {
  PrepareFrame,
  Update,
  RenderFrame,
  EndFrame,
}

pub fn build() -> App {
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

  // Define Game flow
  builder
    .add_plugin(StagePlugin)
    .insert_resource(Clock::new())
    .insert_resource(UserInput::default());

  // Loading Scene
  builder
    .add_state(AppState::Loading)
    // Loading
    .add_system_set(
      SystemSet::on_enter(AppState::Loading)
        .with_system(crate::stage::setup.system())
    )
    .add_system_set(
      SystemSet::on_update(AppState::Loading)
        .with_system(crate::stage::check_setup.system())
    );

    // Game
    // https://github.com/bevyengine/bevy/blob/38feddb87850424df3a0b08bae8dc32c57004798/examples/ecs/system_sets.rs
  builder
    .add_system_set(SystemSet::on_update(AppState::Stage)
      .label(Stage::PrepareFrame)
      .with_system(handle_input_events.system())
    )
    .add_system_set(SystemSet::on_update(AppState::Stage)
      .label(Stage::Update)
      .after(Stage::PrepareFrame)
      .with_system(move_by_motion.system())
      .with_system(Sora::update.system())
    )
    .add_system_set(SystemSet::on_update(AppState::Stage)
      .label(Stage::RenderFrame)
      .after(Stage::Update)
      .with_system(copy_pos_to_transform.system())
      .with_system(handle_entity_vanishing.system())
    )
    .add_system_set(SystemSet::on_update(AppState::Stage)
      .label(Stage::EndFrame)
      .after(Stage::RenderFrame)
      .with_system(control_clock.system())
      .with_system(handle_lifetime.system())
      .with_system(ScenarioSever::update.system())
    );
  builder.app
}
