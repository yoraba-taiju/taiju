use std::ops::DerefMut;
use std::ops::Deref;

use crate::scenes::stage::prelude::*;
use bevy::reflect::TypeUuid;

pub mod loader;
pub use loader::ScenarioLoader;

use anyhow::{Result, Context};
use bevy::asset::Asset;
use crate::donut::SubjectiveTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
  WitchSpeedChanged(Motion),
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

pub struct ScenarioDirector {
  handle: Handle<Scenario>,
  scenario: Option<Scenario>,
  started: u32,
  //
  read_events: Value<usize>,
  spawned_objects: Value<usize>,
  scene_speed: Value<Motion>,
  scene_position: Value<Position>
}

impl ScenarioDirector {
  pub fn spawn(clock: &Res<ClockRef>, asset_server: Res<AssetServer>) -> Self {
    let handle = asset_server.load::<Scenario, _>("scenario/stage01.ron");
    Self{
      handle,
      scenario: None,
      started: 0,
      read_events: clock.value(0),
      spawned_objects: clock.value(0),
      scene_speed: clock.value(Default::default()),
      scene_position: clock.value(Default::default()),
    }
  }
  pub fn init(&mut self, current: SubjectiveTime) {
    self.started = current.ticks;
  }
  pub fn handle(
    &mut self,
    clock: &Res<ClockRef>,
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    sora: &(Entity, &Position),
  ) {
    let scenario = self.scenario.as_ref().unwrap();
    let current = clock.current_tick() - self.started.clone();
    for i in (*self.read_events)..scenario.events.len() {
      let (at, ev) = scenario.events[i].clone();
      if at > current {
        break;
      }
      *self.read_events += 1;
      match ev {
        Event::WitchSpeedChanged(motion) => { *self.scene_speed = motion; }
      }
    }
    self.scene_position.advance(&self.scene_speed);
    let pos = *self.scene_position;
    for i in (*self.spawned_objects)..scenario.objects.len() {
      let obj = &scenario.objects[i];
      (*self.spawned_objects) += 1;
    }
  }
}

pub fn spawn_scenario(
  clock: Res<ClockRef>,
  mut director: ResMut<ScenarioDirector>,
  mut assets: ResMut<Assets<Scenario>>,
) {
  if director.scenario.is_none() {
    director.scenario = assets.remove(&director.handle);
    director.handle = Handle::default();
    director.init(clock.current_time());
  }
}

pub fn progress_scenario(
  clock: Res<ClockRef>,
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  sora_query: Query<(Entity, &Position), With<Sora>>,
  mut director: ResMut<ScenarioDirector>,
) {
  if director.scenario.is_none() {
    return;
  }
  let sora: (Entity, &Position) = sora_query.single().unwrap();
  director.handle(&clock, &mut commands, asset_server, &sora);
}
