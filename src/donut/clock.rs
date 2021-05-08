use super::*;
use std::sync::{Arc, RwLock};
use std::fmt::{Debug, Formatter};

pub type ClockRef = Arc<Clock>;
pub struct Clock {
  state: RwLock<Inner>
}
struct Inner {
  current: SubjectiveTime,
  inspect_at: Option<u32>,
  leap_intersection: Vec<u32>,
  availabe_from: u32,
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
      state: RwLock::new(Inner{
        current: SubjectiveTime{
          leaps: 0,
          ticks: 0,
        },
        inspect_at: None,
        leap_intersection: Vec::new(),
        availabe_from: 0,
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
  pub fn is_inspected(&self) -> bool {
    let state = self.state.read().expect("Failed to lock Clock (read)");
    state.inspect_at.is_some()
  }
  pub fn time_to_read(&self) -> SubjectiveTime {
    let state = self.state.read().expect("Failed to lock Clock (read)");
    if let Some(ticks) = state.inspect_at {
      SubjectiveTime{
        leaps: state.current.ticks,
        ticks,
      }
    } else {
      state.current.clone()
    }
  }
  pub fn ticks_to_read(&self) -> u32 {
    let state = self.state.read().expect("Failed to lock Clock (read)");
    if let Some(ticks) = state.inspect_at {
      ticks
    } else {
      state.current.ticks
    }
  }
  pub fn inspect_at(&self, ticks: u32) {
    let mut state = self.state.write().expect("Failed to lock Clock (write)");
    state.inspect_at = Some(std::cmp::min(std::cmp::max(ticks, state.availabe_from), state.current.ticks));
  }
  pub fn tick(&self) -> SubjectiveTime {
    let mut state = self.state.write().expect("Failed to lock Clock (write)");
    state.current.ticks += 1;
    if (state.availabe_from + (RECORDED_FRAMES as u32)) <= state.current.ticks {
      state.availabe_from = state.current.ticks - (RECORDED_FRAMES as u32) + 1;
    }
    state.current.clone()
  }
  pub(crate) fn leap(&self) -> Option<SubjectiveTime> {
    let mut state = self.state.write().expect("Failed to lock Clock (write)");
    if state.inspect_at.is_none() {
      return None;
    }
    let ticks = state.inspect_at.unwrap();
    if ticks < state.availabe_from || state.current.ticks <= ticks {
      return None;
    }
    state.inspect_at = None;
    state.current.leaps += 1;
    state.current.ticks = ticks;
    // adjust_intersection
    for branch in state.leap_intersection.iter_mut() {
      *branch = std::cmp::min(*branch, ticks);
    }
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
