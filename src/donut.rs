use std::sync::{Arc, Weak, RwLock};
use std::ops::{Deref, DerefMut};
use std::sync::atomic::Ordering;
use bevy::utils::tracing::Instrument;

const RECORD_FRAMES: usize = 600;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
pub struct SubjectiveTime {
  leaps: u32,
  ticks: u32,
}

impl SubjectiveTime {
  fn new(leaps:u32, ticks:u32) -> Self {
    Self {
      leaps,
      ticks,
    }
  }
}

pub struct Clock {
  current: RwLock<SubjectiveTime>,
}

impl Clock {
  fn new() -> Arc<Self> {
    Arc::new(Self {
      current: RwLock::new(SubjectiveTime{
        leaps: 0,
        ticks: 0,
      }),
    })
  }
  fn subjective_time(&self) -> SubjectiveTime {
    let t = self.current.read().expect("Failed to lock Clock (read)");
    t.deref().clone()
  }
  fn objective_time(&self) -> u32 {
    let t = self.current.read().expect("Failed to lock Clock (read)");
    t.ticks
  }
  fn tick(&self) -> u32 {
    let mut t = self.current.write().expect("Failed to lock Clock (write)");
    t.ticks += 1;
    t.ticks
  }
  fn leap(&self, ticks: u32) -> SubjectiveTime {
    let mut t = self.current.write().expect("Failed to lock Clock (write)");
    t.leaps += 1;
    t.ticks = ticks;
    t.clone()
  }
}

struct ValueEntry<T> {
  time: SubjectiveTime,
  value: T,
}

impl <T> ValueEntry<T> {
  fn new(time: SubjectiveTime, value: T) -> Self {
    Self {
      time,
      value,
    }
  }
}

pub struct Value<T> {
  clock: Weak<Clock>,
  history: Vec<ValueEntry<T>>,
  begin: usize,
  end: usize,
}

impl <T> Value<T> {
  fn new(clock: &Arc<Clock>, initial: T) -> Self {
    Self {
      clock: Arc::downgrade(clock),
      history: vec![ValueEntry::new(clock.subjective_time(), initial)],
      begin: 0,
      end: 1,
    }
  }
  pub(crate) fn find_write_index(&self, subjective_time: SubjectiveTime) -> usize {
    let mut beg: usize;
    let mut end: usize;
    let ticks = subjective_time.ticks;
    if self.begin < self.end {
      beg = self.begin;
      end = self.end;
    } else {
      beg = self.begin;
      end = self.end + RECORD_FRAMES;
    }
    while beg < end {
      let mid = beg + (end - beg)/2;
      let mid_t = self.history[mid % RECORD_FRAMES].time.ticks;
      if mid_t < ticks {
        beg = mid + 1;
      } else {
        end = mid - 1;
      }
    }
    return end;
  }
}

impl <T> Deref for Value<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    let clock = self.clock.upgrade().unwrap();
    &self.history[0].value
  }
}

impl <T> DerefMut for Value<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    todo!()
  }
}

#[cfg(test)]
mod test {
  use crate::donut::{Clock, Value, SubjectiveTime};

  #[test]
  fn clock_tick() {
    let mut clock = Clock::new();
    assert_eq!(SubjectiveTime::new(0, 0), clock.subjective_time());
    clock.tick();
    assert_eq!(SubjectiveTime::new(0, 1), clock.subjective_time());
  }
  #[test]
  fn leap_test() {
    let mut clock = Clock::new();
    clock.tick();
    clock.tick();
    clock.leap(1);
    assert_eq!(SubjectiveTime::new(1, 1), clock.subjective_time());
  }

  #[test]
  fn simple_value_test() {
    let mut clock = Clock::new();
    let mut value = Value::<u32>::new(&clock, 0);
    clock.tick();
    clock.tick();
    assert_eq!(1, value.find_write_index(clock.subjective_time()));
    clock.leap(1);
    //assert_eq!(Some(&0u32), value.value());
  }
}