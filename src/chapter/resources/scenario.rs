use std::ops::DerefMut;
use std::ops::Deref;

use crate::chapter::prelude::*;
use bevy::reflect::TypeUuid;

pub mod loader;
pub use loader::ScenarioLoader;

use anyhow::{Result, Context};
use bevy::asset::{Asset, HandleId};
use crate::donut::SubjectiveTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
  ChangeWitchSpeed(Motion),
  SpawnEnemy(EnemyDescription)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Object {
  pub center: (f32, f32),
  pub size: (f32, f32),
  pub image_path: String,
}

#[derive(Default, Debug, Clone, TypeUuid, Serialize, Deserialize)]
#[uuid = "779ba602-ab1a-11eb-bcbc-0242ac130002"]
pub struct Scenario {
  pub events: Vec<(u32, Event)>,
  pub objects: Vec<Object>,
}

impl Scenario {
  pub(crate) fn try_from_bytes(bytes: &[u8]) -> Result<Scenario> {
    let str = std::str::from_utf8(&bytes)?;
    ron::from_str::<Scenario>(str).map_err(anyhow::Error::from)
  }
  pub(crate) fn to_string(&self) -> String {
    ron::ser::to_string(self).unwrap()
  }
}

//

pub struct ScenarioSever {
  scenario_handle: Handle<Scenario>,
  started: u32,
  //
  read_events: Value<usize>,
  spawned_objects: Value<usize>,
  scene_speed: Value<Motion>,
  scene_position: Value<Position>
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
      started: clock.current_ticks(),
      read_events: clock.make(0),
      spawned_objects: clock.make(0),
      scene_speed: clock.make(Default::default()),
      scene_position: clock.make(Default::default()),
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
    asset_server: Res<AssetServer>,
    enemy_server: Res<EnemyServer>,
    sora_query: Query<(Entity, &Value<Position>), With<Sora>>,
  ) {
    let scenario = scenarios.get(&seq.scenario_handle).unwrap();
    let sora: (Entity, &Value<Position>) = sora_query.single().unwrap();
    let current = clock.current_ticks() - seq.started.clone();
    for i in (*seq.read_events)..scenario.events.len() {
      let (at, ev) = scenario.events[i].clone();
      if at > current {
        break;
      }
      *seq.read_events += 1;
      match ev {
        Event::ChangeWitchSpeed(motion) => { *seq.scene_speed = motion; }
        Event::SpawnEnemy(desc) => {
          enemy_server.spawn_enemy(&desc, &clock, &mut commands);
        }
      }
    }
    let speed = *seq.scene_speed;
    seq.scene_position.apply(&speed);
    let pos = *seq.scene_position;
    for i in (*seq.spawned_objects)..scenario.objects.len() {
      let obj = &scenario.objects[i];
      (*seq.spawned_objects) += 1;
    }
  }
}
