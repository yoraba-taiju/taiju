use bevy::prelude::*;
use super::*;
use crate::donut::{Value, ClockRef};
pub struct ScenarioSever {
  scenario_handle: Handle<Scenario>,
  //
  scene_time: Value<u32>,
}

impl ScenarioSever {
  pub fn spawn(
    commands: &mut Commands,
    clock: &Res<ClockRef>,
    asset_server: &Res<AssetServer>,
  ) {
    let handle = asset_server.load::<Scenario, _>("scenario/stage01.ron");
    commands.insert_resource(ScenarioSever{
      scenario_handle: handle,
      scene_time: clock.make(Default::default()),
    });
  }
  pub fn get_asset_handles(&self) -> Vec<HandleId> {
    vec![self.scenario_handle.id]
  }
  pub fn update(
    mut seq: ResMut<ScenarioSever>,
    scenarios: Res<Assets<Scenario>>,
    clock: Res<ClockRef>,
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    enemy_server: Res<EnemyServer>,
  ) {
    if clock.is_inspected() {
      return;
    }
    let scenario = scenarios.get(&seq.scenario_handle).unwrap();
    let current = *seq.scene_time;
    if let Some(velocity) = scenario.course.speed_changes.get(&current) {

    }
    if let Some(events) = scenario.events.get(&current) {
      for ev in events.iter() {
        match ev.clone() {
          Event::SpawnEnemy(desc) => {
            enemy_server.spawn_enemy(&desc, &clock, &mut commands);
          }
          Event::SpawnScape(scape) => {
            todo!()
          }
        }  
      }
    }
    *seq.scene_time = current + 1;
  }
}
