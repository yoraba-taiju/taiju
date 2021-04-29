use bevy::prelude::*;

use crate::taiju::system::UserInput;
use crate::donut::{Clock, ClockRef};

mod taiju;
pub mod donut;

fn main() {
  App::build()
    .insert_resource(WindowDescriptor {
      title: "Yoraba Taiju".to_string(),
      width: 1920.,
      height: 1080.,
      vsync: true,
      ..Default::default()
    })
    .insert_resource::<ClockRef>(Clock::new())
    .add_plugins(DefaultPlugins)
    .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
    .insert_resource(UserInput::default())
    .add_startup_system(taiju::scenes::stage::setup.system())
    .add_system_to_stage(CoreStage::PreUpdate, taiju::system::gamepad_events.system())
    .add_system_to_stage(CoreStage::PreUpdate, taiju::system::keyboard_events.system())
    .add_system(taiju::scenes::stage::systems::move_sora.system())
    .add_system(taiju::scenes::stage::systems::transform.system())
    .run();
}
