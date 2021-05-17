use std::ops::DerefMut;

use crate::prelude::*;
use crate::chapter::states::ScenarioReader;
use crate::chapter::resources::*;
use crate::chapter::scenario::*;

pub fn update(
  mut reader: ResMut<ScenarioReader>,
  clock: Res<ClockRef>,
  mut commands: Commands,
  _asset_server: Res<AssetServer>,
  witch_server: Res<WitchServer>,
  enemy_server: Res<EnemyServer>,
) {
  if clock.is_inspected() {
    return;
  }
  let reader = reader.deref_mut();
  let scenario = &reader.scenario;
  let current = *reader.page;
  if let Some(velocity) = scenario.course.speed_changes.get(&current) {

  }
  if let Some(events) = scenario.events.get(&current) {
    for ev in events.iter() {
      match ev.clone() {
        Event::SpawnWitch(witch_kind, position) => {
          witch_server.spawn(&clock, &mut commands, witch_kind, position);
        }
        Event::SpawnEnemy(desc) => {
          enemy_server.spawn(&desc, &clock, &mut commands);
        }
        Event::SpawnScape(scape) => {
          todo!()
        }
      }  
    }
  }
  *reader.page = current + 1;
}
