pub const RECORDED_FRAMES: usize = 300;

pub mod clock;
pub use clock::ClockRef;
pub use clock::Clock;

pub mod subjective_time;
pub use subjective_time::SubjectiveTime;

pub mod value;
pub use value::Value;

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
    *value = 11;
    assert_eq!(11, *value);
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
  fn value_test_with_long() {
    let clock = Clock::new();
    let mut value = Value::<u32>::new(&clock, 0);
    for i in 0..1000 {
      *value = i;
      clock.tick();
    }
    for i in (700..1000).rev() {
      clock.leap(i);
      assert_eq!(i, *value);
    }
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