use bevy::{
  input::{
    gamepad::{GamepadEvent, GamepadEventType},
    keyboard::KeyboardInput,
  },
  prelude::*,
};
use crate::donut::ClockRef;

#[derive(Default)]
pub struct UserInput {
  pub x: f32,
  pub y: f32,
  pub clock_direction: i8,
}

pub fn handle_input_events(
  mut input: ResMut<UserInput>,
  keyboard_input: Res<Input<KeyCode>>,
  mut gamepad_event: EventReader<GamepadEvent>,
) {
  let mut x: f32 = 0.0;
  let mut y: f32 = 0.0;
  for event in gamepad_event.iter() {
    match &event {
      GamepadEvent(gamepad, GamepadEventType::Connected) => {
        println!("{:?} Connected", gamepad);
      }
      GamepadEvent(gamepad, GamepadEventType::Disconnected) => {
        println!("{:?} Disconnected", gamepad);
      }
      GamepadEvent(gamepad, GamepadEventType::ButtonChanged(button_type, value)) => {
        println!("{:?} of {:?} is changed to {}", button_type, gamepad, value);
      }
      GamepadEvent(gamepad, GamepadEventType::AxisChanged(axis_type, value)) => {
        match axis_type {
          &GamepadAxisType::DPadX => {
            x += *value;
          },
          &GamepadAxisType::DPadY=> {
            y += *value;
          }
          _ => {}
        };
        //println!("{:?} of {:?} is changed to {}", axis_type, gamepad, value);
      }
    }
  }
  if keyboard_input.pressed(KeyCode::Up) {
    y += 1.0;
  }
  if keyboard_input.pressed(KeyCode::Down) {
    y -= 1.0;
  }
  if keyboard_input.pressed(KeyCode::Left) {
    x += -1.0;
  }
  if keyboard_input.pressed(KeyCode::Right) {
    x += 1.0;
  }
  input.clock_direction = 1;
  if keyboard_input.pressed(KeyCode::X) {
    input.clock_direction = -1;
  }
  input.x = x;
  input.y = y;
}

pub fn control_clock(
  input: Res<UserInput>,
  clock: Res<ClockRef>,
)
{
  if input.clock_direction < 0 {
    clock.leap(clock.current_tick() - 1);
  }else{
    clock.tick();
  }
}