use std::sync::{Arc, RwLock};
use super::*;
use std::fmt::{Debug, Formatter};

pub type ClockRef = Arc<Clock>;
pub struct Clock {
  state: RwLock<ClockState>
}
struct ClockState {
  current: SubjectiveTime,
  leap_intersection: Vec<u32>,
  availabe_from: u32,
  inspect_at: Option<u32>,
}

impl Debug for Clock {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let state = self.state.read().unwrap();
    let leap_intersections_str = {
      let inner = state.leap_intersection
        .iter()
        .enumerate()
        .map(|(i,t)| format!("(leap={}, at={})", i, t))
        .collect::<Vec<String>>()
        .join(", ");
      format!("[{}]", inner)
    };
    let current_str = format!("({}, {})", state.current.leaps, state.current.ticks);
    f.write_str(
      format!("Clock {{ current: {}, intersections: {} }}",
              current_str,
              leap_intersections_str
      ).as_str())
  }
}

impl Clock {
  pub fn new() -> ClockRef {
    Arc::new(Clock {
      state: RwLock::new(ClockState{
        current: SubjectiveTime{
          leaps: 0,
          ticks: 0,
        },
        leap_intersection: Vec::new(),
        availabe_from: 0,
        inspect_at: None,
      })
    })
  }
  pub fn make<T: Clone>(self: &Arc<Self>, value: T) -> Value<T> {
    Value::new(self, value)
  }
  pub fn current_time(&self) -> SubjectiveTime {
    let state = self.state.read().expect("Failed to lock Clock (read)");
    state.current.clone()
  }
  pub fn current_ticks(&self) -> u32 {
    let state = self.state.read().expect("Failed to lock Clock (read)");
    state.current.ticks
  }
  pub fn tick(&self) -> SubjectiveTime {
    let mut state = self.state.write().expect("Failed to lock Clock (write)");
    state.current.ticks += 1;
    if (state.availabe_from + (RECORDED_FRAMES as u32)) <= state.current.ticks {
      state.availabe_from = state.current.ticks - (RECORDED_FRAMES as u32);
    }
    state.current.clone()
  }
  fn adjust_intersection(leap_intersection: &mut Vec<u32>, leap_ticks: u32) {
    for branch in leap_intersection.iter_mut() {
      if leap_ticks < *branch {
        *branch = leap_ticks;
      }
    }
  }
  pub(crate) fn leap(&self, ticks: u32) -> Option<SubjectiveTime> {
    let mut state = self.state.write().expect("Failed to lock Clock (write)");
    if ticks < state.availabe_from || state.current.ticks <= ticks {
      return None;
    }
    state.current.leaps += 1;
    state.current.ticks = ticks;
    Self::adjust_intersection(&mut state.leap_intersection, ticks);
    state.leap_intersection.push(ticks);
    Some(state.current.clone())
  }
  pub(crate) fn adjust_read_time(&self, last_modified_leaps: u32, ticks_to_read: u32) -> u32 {
    let state = self.state.read().expect("Failed to lock Clock (read)");
    if last_modified_leaps == state.current.leaps {
      ticks_to_read
    } else {
      std::cmp::min(state.leap_intersection[(last_modified_leaps) as usize], ticks_to_read)
    }
  }
}
