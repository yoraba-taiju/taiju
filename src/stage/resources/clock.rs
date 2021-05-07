use crate::stage::prelude::*;

pub fn control_clock(
  input: Res<UserInput>,
  clock: Res<ClockRef>,
)
{
  if input.clock_direction < 0 {
    // TODO: フレームでリミッターをかける
    if clock.current_ticks() > 0 {
      clock.leap(clock.current_ticks() - 1);
    }
  }else{
    clock.tick();
  }
}
