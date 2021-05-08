pub const RECORDED_FRAMES: usize = 300;

mod subjective_time;
pub use subjective_time::SubjectiveTime;

pub mod clock;
pub use clock::ClockRef;
pub use clock::Clock;

pub mod value;
pub use value::Value;

#[cfg(test)]
mod test {
  use crate::donut::{Clock, Value, SubjectiveTime, RECORDED_FRAMES};
  #[cfg(test)]
  pub(crate) fn new_subjective(leaps: u32, ticks: u32) -> SubjectiveTime {
    SubjectiveTime {
      leaps,
      ticks,
    }
  }


  #[test]
  fn clock_tick() {
    let clock = Clock::new();
    assert_eq!(new_subjective(0, 0), clock.current_time());
    clock.tick();
    assert_eq!(new_subjective(0, 1), clock.current_time());
  }

  #[test]
  fn leap_test() {
    let clock = Clock::new();
    clock.tick();
    clock.tick();
    clock.inspect_at(1);
    clock.leap();
    assert_eq!(new_subjective(1, 1), clock.current_time());
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
    clock.inspect_at(1);
    clock.leap();
    assert_eq!(100, *value);
    clock.tick();
    assert_eq!(100, *value);
  }
  #[test]
  fn check_capacity_and_len() {
    let clock = Clock::new();
    let  mut value = Value::<u32>::new(&clock, 0);
    assert_eq!(1, value.len());
    assert!(RECORDED_FRAMES <= value.capacity());
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
    clock.inspect_at(1);
    clock.leap();
    assert_eq!(1, *value);
    clock.tick(); // tick = 2
    clock.tick(); // tick = 3
    clock.inspect_at(1);
    clock.leap();
    assert_eq!(1, *value);
    clock.tick(); // tick = 2
    *value = 22;
    clock.tick(); // tick = 3
    clock.inspect_at(2);
    clock.leap();
    assert_eq!(22, *value);
    clock.inspect_at(1);
    clock.leap();
    assert_eq!(1, *value);
    *value = 11;
    assert_eq!(11, *value);
    clock.inspect_at(0);
    clock.leap();
    assert_eq!(0, *value);
  }
  #[test]
  fn value_test_with_single() {
    let clock = Clock::new();
    let mut value = Value::<u32>::new(&clock, 0);
    *value = 1;
    clock.tick(); // tick = 1
    clock.inspect_at(0);
    clock.leap();
    *value = 2;
    assert_eq!(2, *value);
  }
  #[test]
  fn value_test_with_long() {
    let clock = Clock::new();
    let mut value = Value::<u32>::new(&clock, 0);
    for i in 0..1000 {
      *value = i;
      clock.tick();
    }
    for i in (701..1000).rev() {
      clock.inspect_at(i);
      clock.leap();
      assert_eq!(i, *value);
    }
  }
  #[test]
  #[should_panic]
  fn read_future_value() {
    let clock = Clock::new();
    clock.tick(); // ticks = 1
    let value = Value::<u32>::new(&clock, 0);
    clock.inspect_at(0);
    clock.leap();
    let _unused = *value;
  }
  #[test]
  #[should_panic]
  fn write_future_value() {
    let clock = Clock::new();
    clock.tick(); // ticks = 1
    let mut value = Value::<u32>::new(&clock, 0);
    clock.inspect_at(0);
    clock.leap();
    *value = 10;
  }
}