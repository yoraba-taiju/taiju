use std::ops::{Deref, DerefMut};
use bevy::utils::HashMap;
use std::any::{TypeId, Any};
use arr_macro::arr;
use std::mem::swap;
use std::marker::PhantomData;

const RECORD_FRAMES: usize = 600;

struct Ticket<T:'static> {
  storage_index: usize,
  value_index: usize,
  _phantom_data: PhantomData<T>,
}

impl Ticket<T> {
  fn new(storage_index: usize, value_index: usize) -> Self {
    Self {
      storage_index,
      value_index,
      _phantom_data: Default::default(),
    }
  }
}

#[derive(Default)]
struct ValueStorage<T> {
  values: Vec<T>,
}

#[derive(Default)]
struct Snapshot {
  time: f32,
  storages: Vec<Box<dyn Any>>,
}

impl Snapshot {
  fn new(time: f32) -> Self {
    Self {
      time,
      storages: Default::default(),
    }
  }

  pub(crate) fn put<T:'static>(&mut self, storage_index: usize, value: T) -> usize {
    let storage = self.storage_mut(storage_index);

  }

  fn storage<T:'static>(&self, idx: usize) -> &Box<ValueStorage<T>> {
    let storage = &self.storages[idx];
    storage.downcast_ref::<Box<ValueStorage<T>>>().expect("Assertion error")
  }
  fn storage_mut<T:'static>(&mut self, idx: usize) -> &mut Box<ValueStorage<T>> {
    let storage = &mut self.storages[idx];
    storage.downcast_mut::<Box<ValueStorage<T>>>().expect("Assertion error")
  }
}

pub struct Store {
  current_frame: usize,
  type_indexes: HashMap<TypeId, usize>,
  snapshots: Vec<Snapshot>,
}

impl Store {
  pub fn new() -> Self {
    Store {
      current_frame: 0,
      type_indexes: Default::default(),
      snapshots: (0..RECORD_FRAMES).map(|_n| Snapshot::default()).collect(),
    }
  }
  fn storage_index_of<T:'static>(&self) -> Option<usize> {
    let type_id = TypeId::of::<T>();
    self.type_indexes.get(&type_id).map(|n| *n)
  }
  fn snapshot_index(&self) -> usize {
    self.current_frame % RECORD_FRAMES
  }
  pub fn tick(&mut self, time: f32) {
    self.current_frame += 1;
    let idx = self.snapshot_index();
    self.snapshots[idx] = Snapshot::new(time);
  }
  pub fn leap(&mut self, frame: usize) -> Result<(), &'static str> {
    if self.current_frame <= frame + RECORD_FRAMES && frame <= self.current_frame {
      self.current_frame = frame;
      Ok(())
    } else {
      Err("Invalid range")
    }
  }
  pub fn current_frame(&self) -> usize {
    self.current_frame
  }

  pub fn put<T:'static>(&mut self, value: T) -> Ticket<T> {
    let snapshot = &mut self.snapshots[self.snapshot_index()];
    if Some(idx) = self.storage_index_of::<T>() {
      snapshot.
    } else {

    }
  }
  pub fn get<T:'static>(&self, ticket: &Ticket<T>) -> &T {

  }
  pub fn get_mut<T:'static>(&mut self, ticket: &Ticket<T>) -> &mut T {

  }
}

#[derive(Clone)]
pub struct Value<'a, V>
  where V: Default + Copy
{
  clock: &'a Store,
  values: [V; RECORD_FRAMES],
  begin_frame: usize,
  last_frame: usize,
}

impl <'a, V> Value<'a, V>
  where V:Default + Copy
{
  fn new(clock: &'a Store) -> Self {
    Self {
      clock,
      values: [V::default(); RECORD_FRAMES],
      begin_frame: clock.current_frame,
      last_frame: clock.current_frame,
    }
  }
  fn index(&self) -> Option<usize> {
    let index = self.clock.current_frame - self.begin_frame;
    let last_index = self.last_frame - self.begin_frame;
    if (last_index as isize - index as isize).abs() > RECORD_FRAMES as isize {
      None
    } else {
      if index < last_index {
        Some(index % RECORD_FRAMES)
      } else {
        Some(last_index % RECORD_FRAMES)
      }
    }
  }
  pub fn value(&self) -> Option<&V> {
    self.index().map(|idx| &self.values[idx])
  }
  fn value_mut(&mut self) -> Option<&mut V> {
    if let Some(idx) = self.index() {
      Some(&mut self.values[idx])
    } else {
      None
    }
  }
}

#[cfg(test)]
mod test {
  use crate::donut::{Store, Value};

  #[test]
  fn clock_tick() {
    let mut clock = Store::new();
    assert_eq!(0, clock.current_frame());
    clock.tick(0.1);
    assert_eq!(1, clock.current_frame());
  }
  #[test]
  fn leap_test() {
    let mut clock = Store::new();
    clock.tick(0.1);
    clock.tick(0.2);
    clock.leap(1);
    assert_eq!(1, clock.current_frame());
  }

  #[test]
  fn simple_value_test() {
    let mut clock = Store::new();
    //let mut value = Value::<u32>::new(&clock);
    clock.tick(0.1);
    clock.tick(0.2);
    clock.leap(1);
    //assert_eq!(Some(&0u32), value.value());
  }
}