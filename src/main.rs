use bevy::prelude::*;

use system::UserInput;

use crate::donut::{Clock, ClockRef};

mod donut;
mod system;
mod scenes;

fn main() {
  App::build()
    .insert_resource(WindowDescriptor {
      title: "Yoraba Taiju".to_string(),
      width: 1920.,
      height: 1080.,
      vsync: true,
      ..Default::default()
    })
    .insert_resource(Clock::new())
    .add_plugins(DefaultPlugins)
    .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
    .insert_resource(UserInput::default())
    .add_startup_system(scenes::stage::setup.system())
    .add_system_to_stage(CoreStage::PreUpdate, system::handle_input_events.system())
    .add_system_to_stage(CoreStage::PreUpdate, system::control_clock.system())
    .add_system(scenes::stage::systems::scenario::progress_scenario.system())
    .add_system(scenes::stage::systems::move_sora.system())
    .add_system(scenes::stage::systems::transform.system())
    .add_system(scenes::stage::systems::handle_vanished_entities.system())
    .run();
}
