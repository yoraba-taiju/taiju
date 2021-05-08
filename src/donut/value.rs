use super::*;
use std::ops::{Deref, DerefMut};
use std::fmt::{Debug, Formatter};
use std::collections::VecDeque;
use std::sync::{Arc, Weak};
use std::cmp::min;

pub struct Value<T: Clone> {
  clock: Weak<Clock>,
  last_modified_leaps: u32,
  begin_ticks: u32,
  history: VecDeque<T>,
}

impl <T: Debug + Clone> Debug for Value<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let history_str = {
      let inner = self.history
        .iter()
        .zip((self.begin_ticks as usize)..(self.begin_ticks as usize + self.history.len()))
        .map(|(v, ticks)| format!("{}: {:?}", ticks, v))
        .collect::<Vec<String>>()
        .join(", ");
      format!("[{}]", inner)
    };
    f.write_str(&format!("Value: {}", &history_str))
  }
}

impl <T: Clone> Value<T> {
  pub(crate) fn new(clock: &Arc<Clock>, initial: T) -> Self {
    let current_time = clock.current_time();
    let mut s = Self {
      clock: Arc::downgrade(clock),
      last_modified_leaps: current_time.leaps,
      begin_ticks: current_time.ticks,
      history: VecDeque::new(),
    };
    s.history.reserve(RECORDED_FRAMES);
    s.history.push_back(initial);
    s
  }
  #[cfg(test)]
  pub(crate) fn capacity(&self) -> usize {
    self.history.capacity()
  }
  #[cfg(test)]
  pub(crate) fn len(&self) -> usize {
    self.history.len()
  }

  fn find_read_index(&self, clock: &Arc<Clock>, ticks: u32) -> Option<usize> {
    let time = clock.adjust_read_time(self.last_modified_leaps, ticks);
    if time < self.begin_ticks {
      return None;
    }
    return Some(min((time - self.begin_ticks) as usize, self.history.len() - 1));
  }
}

impl <T: Clone> Deref for Value<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    let clock = self.clock.upgrade().unwrap();
    let idx = self.find_read_index(&clock, clock.ticks_to_read()).expect("Don't read a value in the future!");
    &self.history[idx]
  }
}

impl <T: Clone> DerefMut for Value<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    let clock = self.clock.upgrade().expect("Clock was missing.");
    if clock.is_inspected() {
      panic!("Don't write to values when clock under inspection.");
    }
    let current_time = clock.current_time();
    if self.begin_ticks > current_time.ticks {
      panic!("Don't write into a value in the future!");
    }
    self.last_modified_leaps = current_time.leaps;
    let write_index = (current_time.ticks - self.begin_ticks) as usize;
    let prev = {
      let read_index = self.find_read_index(&clock, current_time.ticks).expect("[BUG] FIXME");
      if read_index == write_index {
        self.history.truncate(write_index + 1);
        return &mut self.history[write_index];
      }
      self.history[read_index].clone()
    };
    if write_index < self.history.len() {
      self.history.truncate(write_index + 1);
      self.history[write_index] = prev;
      return &mut self.history[write_index];
    }
    if write_index < RECORDED_FRAMES {
      self.history.resize(write_index + 1, prev);
      return &mut self.history[write_index];
    }
    let remove_len = write_index + 1 - RECORDED_FRAMES;
    if remove_len >= self.history.len() {
      self.history.clear();
      self.history.push_back(prev);
      self.begin_ticks = current_time.ticks;
      return &mut self.history[0];
    }
    for _ in 0..remove_len {
      self.history.pop_front();
    }
    self.history.resize(RECORDED_FRAMES, prev);
    self.begin_ticks = current_time.ticks - (RECORDED_FRAMES as u32) + 1;
    &mut self.history[RECORDED_FRAMES - 1]
  }
}
