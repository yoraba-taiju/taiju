use std::sync::{Arc, RwLock};
use super::*;
use std::fmt::{Debug, Formatter};

pub type ClockRef = Arc<Clock>;
pub struct Clock {
  current: RwLock<SubjectiveTime>,
  leap_intersection: RwLock<Vec<u32>>,
}

impl Debug for Clock {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let current = self.current.read().unwrap();
    let leap_intersections_str = {
      let inner = self.leap_intersection.read().unwrap()
        .iter()
        .enumerate()
        .map(|(i,t)| format!("(leap={}, at={})", i, t))
        .collect::<Vec<String>>()
        .join(", ");
      format!("[{}]", inner)
    };
    let current_str = format!("({}, {})", current.leaps, current.ticks);
    f.write_str(
      format!("Clock {{ current: {}, intersections: {} }}",
              current_str,
              leap_intersections_str
      ).as_str())
  }
}

impl Clock {
  pub fn new() -> ClockRef {
    Arc::new(Self {
      current: RwLock::new(SubjectiveTime{
        leaps: 0,
        ticks: 0,
      }),
      leap_intersection: RwLock::new(Vec::new()),
    })
  }
  pub fn make<T: Clone>(self: &Arc<Self>, value: T) -> Value<T> {
    Value::new(self, value)
  }
  pub fn current_time(&self) -> SubjectiveTime {
    let t = self.current.read().expect("Failed to lock Clock (read)");
    t.clone()
  }
  pub fn current_tick(&self) -> u32 {
    let t = self.current.read().expect("Failed to lock Clock (read)");
    t.ticks
  }
  pub fn tick(&self) -> u32 {
    let mut t = self.current.write().expect("Failed to lock Clock (write)");
    t.ticks += 1;
    t.ticks
  }
  fn adjust_intersection(leap_intersection: &mut Vec<u32>, leap_ticks: u32) {
    for branch in leap_intersection.iter_mut() {
      if leap_ticks < *branch {
        *branch = leap_ticks;
      }
    }
  }
  pub(crate) fn leap(&self, ticks: u32) -> SubjectiveTime {
    let mut current = self.current.write().expect("Failed to lock Clock (write)");
    if ticks == current.ticks {
      return current.clone();
    }
    let mut leap_intersection = self.leap_intersection.write().expect("Failed to lock intersection");
    current.leaps += 1;
    current.ticks = ticks;
    Self::adjust_intersection(&mut leap_intersection, ticks);
    leap_intersection.push(ticks);
    current.clone()
  }
  pub(crate) fn adjust_read_time(&self, value_time: SubjectiveTime) -> u32 {
    let current = self.current.read().expect("Failed to lock Clock (write)");
    let leap_intersection = self.leap_intersection.read().expect("Failed to lock intersection");
    if value_time.leaps == current.leaps {
      value_time.ticks
    } else {
      std::cmp::min(leap_intersection[(value_time.leaps) as usize], value_time.ticks)
    }
  }
}
