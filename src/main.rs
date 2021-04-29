use bevy::prelude::*;

mod donut;
mod system;
mod scenes;

fn main() {
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
    use crate::scenes::stage::prelude::*;
    use crate::donut::Clock;
    builder
      .insert_resource(Clock::new())
      .insert_resource(UserInput::default())
      .add_startup_system(scenes::stage::setup.system())
      .add_system_to_stage(CoreStage::PreUpdate, handle_input_events.system())
      .add_system_to_stage(CoreStage::PreUpdate, control_clock.system())
      .add_system(progress_scenario.system())
      .add_system(move_sora.system())
      .add_system(copy_to_transform.system())
      .add_system(handle_vanished_entities.system());
  }
  builder.run();
}
