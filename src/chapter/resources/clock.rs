use crate::chapter::prelude::*;

pub fn control_clock(
  input: Res<UserInput>,
  clock: Res<ClockRef>,
)
{
  if input.clock_direction < 0 {
    let ticks = clock.ticks_to_read();
    if ticks > 0 {
      clock.inspect_at(ticks - 1);
    }
  }else{
    if clock.is_inspected() {
      clock.leap();
    } else {
      clock.tick();
    }
  }
}
