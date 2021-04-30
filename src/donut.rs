use std::sync::{Arc, Weak, RwLock};
use std::ops::{Deref, DerefMut};
use std::cmp::min;
use std::fmt::{Debug, Formatter};
//use typenum::{UInt, UTerm};
//use typenum::bit::{B0, B1};
use heapless::consts::U300;

pub const RECORDED_FRAMES: usize = 300;
//0b11100001000
//type RecordFramesTNum = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>;
type RecordFramesTNum = U300;

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
        .map(|(i,t)| format!("(idx={}, time={})", i, t))
        .collect::<Vec<String>>()
        .join(", ");
      format!("[{}]", inner)
    };

    f.debug_struct("Clock")
      .field("current leaps", &current.leaps)
      .field("current ticks", &current.ticks)
      .field("leap intersections", &leap_intersections_str)
      .finish()
  }
}

impl Clock {
  pub(crate) fn new() -> ClockRef {
    Arc::new(Self {
      current: RwLock::new(SubjectiveTime{
        leaps: 0,
        ticks: 0,
      }),
      leap_intersection: RwLock::new(Vec::new()),
    })
  }
  pub fn value<T: Clone>(self: &Arc<Self>, value: T) -> Value<T> {
    Value::new(self, value)
  }
  pub fn current_time(&self) -> SubjectiveTime {
    let t = self.current.read().expect("Failed to lock Clock (read)");
    t.deref().clone()
  }
  pub fn current_tick(&self) -> u32 {
    let t = self.current.read().expect("Failed to lock Clock (read)");
    t.deref().ticks
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
    let mut leap_intersection = self.leap_intersection.write().expect("Failed to lock intersection");
    if let Some(last_ticks) = leap_intersection.last() {
      // optimized path.
      if ticks <= *last_ticks {
        current.ticks = ticks;
        Self::adjust_intersection(&mut leap_intersection, ticks);
        return current.clone();
      }
    }
    current.leaps += 1;
    current.ticks = ticks;
    Self::adjust_intersection(&mut leap_intersection, ticks);
    leap_intersection.push(ticks);
    current.clone()
  }
  fn adjust_read_time(&self, value_time: SubjectiveTime) -> u32 {
    let current = self.current.read().expect("Failed to lock Clock (write)");
    let leap_intersection = self.leap_intersection.read().expect("Failed to lock intersection");
    if value_time.leaps == current.leaps {
      value_time.ticks
    } else {
      min(leap_intersection[(value_time.leaps) as usize], value_time.ticks)
    }
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

impl <T: Debug> Debug for ValueEntry<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("ValueEntry")
      .field("time", &self.time)
      .field("value", &self.value)
      .finish()
  }
}
pub struct Value<T: Clone> {
  clock: Weak<Clock>,
  history: heapless::Vec<ValueEntry<T>, RecordFramesTNum>,
}

impl <T: Debug + Clone> Debug for Value<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let history_str = {
      let inner = self.history
        .iter()
        .map(|entry| format!("([{}, {}] = {:?})", &entry.time.leaps, &entry.time.ticks, &entry.value))
        .collect::<Vec<String>>()
        .join(", ");
      format!("[{}]", inner)
    };
    f.write_str(&format!("Value: {}", &history_str))
  }
}

impl <T: Clone> Value<T> {
  fn new(clock: &Arc<Clock>, initial: T) -> Self {
    let mut s = Self {
      clock: Arc::downgrade(clock),
      history: heapless::Vec::new(),
    };
    s.history.push(ValueEntry::new(clock.current_time(), initial)).ok().expect("FIXME");
    s
  }
  pub(crate) fn find_write_index(&self, subjective_time: SubjectiveTime) -> usize {
    let ticks = subjective_time.ticks;
    let mut beg: usize = 0;
    let mut end: usize = self.history.len();
    if self.history[end-1].time <= subjective_time {
      return end;
    }
    while beg < end {
      let mid = beg + (end - beg)/2;
      let mid_t = self.history[mid].time.ticks;
      if mid_t < ticks {
        beg = mid + 1;
      } else {
        end = mid;
      }
    }
    return end;
  }
  pub(crate) fn find_read_index(&self, adjusted_time: u32) -> Option<usize> {
    let mut beg: usize = 0;
    let mut end: usize = self.history.len();
    if self.history[end-1].time.ticks <= adjusted_time {
      return Some(end-1);
    }
    while beg < end {
      let mid = beg + (end - beg)/2;
      let mid_t = self.history[mid].time.ticks;
      if mid_t <= adjusted_time {
        beg = mid + 1;
      } else {
        end = mid;
      }
    }
    if end == 0 {
      None
    } else {
      Some(beg - 1)
    }
  }
  #[cfg(test)]
  pub(crate) fn capacity(&self) -> usize {
    self.history.capacity()
  }
  #[cfg(test)]
  pub(crate) fn len(&self) -> usize {
    self.history.len()
  }
}

impl <T: Clone> Deref for Value<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    let clock = self.clock.upgrade().unwrap();
    let time = clock.adjust_read_time(self.history.last().unwrap().time);
    let idx = self.find_read_index(time).unwrap();
    if idx == self.history.len() {
      panic!("Do not refer non-existent value!")
    }
    &self.history[idx].value
  }
}

impl <T: Clone> DerefMut for Value<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    let clock = self.clock.upgrade().expect("Clock was missing.");
    let time = clock.current_time();
    let idx = self.find_write_index(time);
    if idx == self.history.capacity() {
      let latest_value = self.history[idx - 1].value.clone();
      self.history.pop();
      self.history.push(ValueEntry::new(time, latest_value)).ok().expect("FIXME");
      &mut (self.history[idx - 1].value)
    }else if idx == self.history.len() {
      self.history.push(ValueEntry::new(time, self.history[idx - 1].value.clone())).ok().expect("FIXME");
      &mut (self.history[idx].value)
    }else{
      if idx + 1 < self.history.len() {
        self.history.truncate(idx + 1);
      }
      let (current, pasts) = self.history.split_last_mut().unwrap();
      if time.ticks < current.time.ticks {
        panic!("Do not refer non-existent value!")
      }
      if current.time.ticks < time.ticks {
        current.value = pasts[pasts.len()-1].value.clone();
      }
      current.time = time;
      &mut (current.value)
    }
  }
}

#[cfg(test)]
mod test {
  use crate::donut::{Clock, Value, SubjectiveTime, RECORDED_FRAMES};

  #[test]
  fn clock_tick() {
    let clock = Clock::new();
    assert_eq!(SubjectiveTime::new(0, 0), clock.current_time());
    clock.tick();
    assert_eq!(SubjectiveTime::new(0, 1), clock.current_time());
  }

  #[test]
  fn leap_test() {
    let clock = Clock::new();
    clock.tick();
    clock.tick();
    clock.leap(1);
    assert_eq!(SubjectiveTime::new(1, 1), clock.current_time());
  }

  #[test]
  fn simple_value_test() {
    let clock = Clock::new();
    let mut value = Value::<u32>::new(&clock, 0);
    *value = 10;
    assert_eq!(10, *value);
    clock.tick();
    *value = 100;
    clock.tick();
    assert_eq!(100, *value);
    clock.leap(1);
    assert_eq!(100, *value);
    clock.tick();
    assert_eq!(100, *value);
  }
  #[test]
  fn check_capacity_and_len() {
    let clock = Clock::new();
    let  mut value = Value::<u32>::new(&clock, 0);
    assert_eq!(1, value.len());
    assert_eq!(RECORDED_FRAMES, value.capacity());
    *value = 1;
    assert_eq!(1, value.len());
    clock.tick();
    assert_eq!(1, value.len());
    *value = 2;
    assert_eq!(2, value.len());
  }
  #[test]
  fn value_test_with_leap() {
    let clock = Clock::new();
    let mut value = Value::<u32>::new(&clock, 0);
    clock.tick(); // tick = 1
    *value = 1;
    clock.tick(); // tick = 2
    *value = 2;
    clock.leap(1); // tick = 1
    assert_eq!(1, *value);
    clock.tick(); // tick = 2
    clock.tick(); // tick = 3
    clock.leap(1); // tick = 1
    assert_eq!(1, *value);
    clock.tick(); // tick = 2
    *value = 22;
    clock.tick(); // tick = 3
    clock.leap(2); // tick = 2
    assert_eq!(22, *value);
    clock.leap(1); // tick = 1
    assert_eq!(1, *value);
    clock.leap(0); // tick = 1
    assert_eq!(0, *value);
  }
  #[test]
  fn value_test_with_single() {
    let clock = Clock::new();
    let mut value = Value::<u32>::new(&clock, 0);
    *value = 1;
    clock.tick(); // tick = 1
    clock.leap(0); // leap = 1, ticks = 0
    *value = 2;
    assert_eq!(2, *value);
  }
  #[test]
  #[should_panic]
  fn read_future_value() {
    let clock = Clock::new();
    clock.tick(); // ticks = 1
    let value = Value::<u32>::new(&clock, 0);
    clock.leap(0); // ticks = 0
    let _unused = *value;
  }
  #[test]
  #[should_panic]
  fn write_future_value() {
    let clock = Clock::new();
    clock.tick(); // ticks = 1
    let mut value = Value::<u32>::new(&clock, 0);
    clock.leap(0); // ticks = 0
    *value = 10;
  }
}