use std::collections::HashMap;
use crate::prelude::*;
use anyhow::Result;
use bevy::reflect::TypeUuid;

/******************************************************************************
 ** Conds
 ******************************************************************************/

 #[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Condition {
  Always,
  WhenFlagSet(String),
  WhenFlagNotSet(String),
}

impl Default for Condition {
  fn default() -> Self {
    Condition::Always
  }
}

/******************************************************************************
 ** Course
 ******************************************************************************/

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Course {
  pub length: u32,
  pub speed_changes: HashMap<u32, Velocity>,
}

/******************************************************************************
 ** Events
 ******************************************************************************/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
  SpawnWitch(WitchKind, Position),
  SpawnEnemy(EnemyKind, EnemyAttackKind, Position),
  SpawnScape(ScapeDescription),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnemyDescription {
  pub enemy: EnemyKind,
  pub attack: EnemyAttackKind,
  pub position: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScapeDescription {
  pub scape: ScapeKind,
  pub position: Position,
}

/******************************************************************************
 ** Scenario
 ******************************************************************************/

#[derive(Default, Debug, Clone, TypeUuid, Serialize, Deserialize)]
#[uuid = "779ba602-ab1a-11eb-bcbc-0242ac130002"]
pub struct Scenario {
  pub course: Course,
  pub events: HashMap<u32, Vec<Event>>,
}

impl Scenario {
  pub fn try_from_bytes(bytes: &[u8]) -> Result<Scenario> {
    let str = std::str::from_utf8(&bytes)?;
    ron::from_str::<Scenario>(str).map_err(anyhow::Error::from)
  }
  pub fn to_string(&self) -> String {
    ron::ser::to_string(self).unwrap()
  }
}
