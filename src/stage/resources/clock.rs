use crate::stage::prelude::*;

pub fn control_clock(
  input: Res<UserInput>,
  clock: Res<ClockRef>,
)
{
  if input.clock_direction < 0 {
    // TODO: フレームでリミッターをかける
    if clock.current_tick() > 0 {
      clock.leap(clock.current_tick() - 1);
    }
  }else{
    clock.tick();
  }
}
