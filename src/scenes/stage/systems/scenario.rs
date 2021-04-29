use bevy::prelude::*;
use crate::donut::ClockRef;
use crate::scenes::stage::components::*;
use crate::system::UserInput;

pub struct ScenarioDirector {

}

impl ScenarioDirector {
  pub fn load(asset_server: Res<AssetServer>) -> Self {
    Self{
    }
  }
}

pub fn progress_scenario(
  mut commands: Commands,
  clock: Res<ClockRef>,
  user_input: Res<UserInput>,
  mut director: ResMut<ScenarioDirector>,
) {
  if user_input.clock_direction <= 0 {
    return;
  }

}