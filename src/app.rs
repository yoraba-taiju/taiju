use bevy::prelude::*;
use crate::stage::prelude::*;
use crate::donut::Clock;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
  Loading,
  Stage,
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
  {
    builder
      .add_plugin(StagePlugin)
      .insert_resource(Clock::new())
      .insert_resource(UserInput::default())

      .add_state(AppState::Loading)
      // Loading
      .add_system_set(
        SystemSet::on_enter(AppState::Loading)
          .with_system(crate::stage::setup.system())
      )
      .add_system_set(
        SystemSet::on_update(AppState::Loading)
          .with_system(crate::stage::check_setup.system())
      )
      // Game
      // TODO: ordering
      // https://github.com/bevyengine/bevy/blob/38feddb87850424df3a0b08bae8dc32c57004798/examples/ecs/system_sets.rs
      .add_system_set_to_stage(CoreStage::Update,
                               SystemSet::on_update(AppState::Stage)
                                 .with_system(handle_input_events.system())
                                 .with_system(control_clock.system()))
      .add_system_set_to_stage(CoreStage::Update,
                               SystemSet::on_update(AppState::Stage)
                                 .with_system(ScenarioSever::update.system())
                                 .with_system(move_by_motion.system())
                                 .with_system(Sora::update.system()))
      .add_system_set_to_stage(CoreStage::PostUpdate,
                               SystemSet::on_update(AppState::Stage)
                                 .with_system(copy_pos_to_transform.system())
                                 .with_system(handle_lifetime.system())
                                 .with_system(handle_entity_vanishing.system()))
    ;
  }
  builder.app
}
