use super::*;
use std::ops::{Deref, DerefMut};
use std::fmt::{Debug, Formatter};
use std::collections::VecDeque;
use std::sync::{Arc, Weak};

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
  history: VecDeque<ValueEntry<T>>,
}

impl <T: Debug + Clone> Debug for Value<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let history_str = {
      let inner = self.history
        .iter()
        .map(|entry| format!("(({}, {}) = {:?})", &entry.time.leaps, &entry.time.ticks, &entry.value))
        .collect::<Vec<String>>()
        .join(", ");
      format!("[{}]", inner)
    };
    f.write_str(&format!("Value: {}", &history_str))
  }
}

impl <T: Clone> Value<T> {
  pub(crate) fn new(clock: &Arc<Clock>, initial: T) -> Self {
    let mut s = Self {
      clock: Arc::downgrade(clock),
      history: VecDeque::new(),
    };
    s.history.reserve_exact(RECORDED_FRAMES);
    s.history.push_back(ValueEntry::new(clock.current_time(), initial));
    s
  }
  pub(crate) fn find_write_index(&self, subjective_time: SubjectiveTime) -> usize {
    let ticks = subjective_time.ticks;
    let mut beg: usize = 0;
    let mut end: usize = self.history.len();
    // Optimization path
    let last_ticks = &self.history[end-1].time.ticks;
    if *last_ticks < subjective_time.ticks {
      return end;
    }
    if *last_ticks == subjective_time.ticks {
      return end - 1;
    }
    while beg < end {
      let mid = beg + (end - beg)/2;
      let mid_t = &self.history[mid].time.ticks;
      if *mid_t < ticks {
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
    // Optimization path
    if self.history[end-1].time.ticks <= adjusted_time {
      return Some(end-1);
    }
    while beg < end {
      let mid = beg + (end - beg)/2;
      let mid_t = &self.history[mid].time.ticks;
      if *mid_t <= adjusted_time {
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
    let time = clock.adjust_read_time(self.history[self.history.len() - 1].time);
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
    let current_time = clock.current_time();
    let idx = self.find_write_index(current_time);
    if idx == RECORDED_FRAMES {
      let latest_value = self.history[idx - 1].value.clone();
      self.history.pop_front();
      self.history.push_back(ValueEntry::new(current_time, latest_value));
      &mut (self.history[idx - 1].value)
    } else if idx == self.history.len() {
      self.history.push_back(ValueEntry::new(current_time, self.history[idx - 1].value.clone()));
      &mut (self.history[idx].value)
    } else {
      if idx + 1 < self.history.len() {
        self.history.truncate(idx + 1);
      }
      if self.history.len() == 1 {
        let latest_ticks = &self.history.front().unwrap().time.ticks;
        if *latest_ticks != current_time.ticks {
          panic!("Do not refer non-existent value!")
        }
      } else {
        let prev_time = self.history[idx - 1].time.clone();
        if prev_time.ticks != current_time.ticks {
          let prev_value = self.history[idx - 1].value.clone();
          self.history[idx].value = prev_value;
        }
      }
      self.history[idx].time = current_time;
      &mut (self.history[idx].value)
    }
  }
}
