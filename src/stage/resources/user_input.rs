use bevy::{
  input::{
    gamepad::{GamepadEvent, GamepadEventType},
  },
  prelude::*,
};
use crate::stage::prelude::*;

#[derive(Default)]
pub struct UserInput {
  pub pad_x: f32,
  pub pad_y: f32,
  pub x: f32,
  pub y: f32,
  pub clock_direction: i8,
}

pub fn handle_input_events(
  mut input: ResMut<UserInput>,
  keyboard_input: Res<Input<KeyCode>>,
  mut gamepad_event: EventReader<GamepadEvent>,
) {
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
            input.pad_x = value.clone();
          },
          &GamepadAxisType::DPadY=> {
            input.pad_y = value.clone();
          }
          _ => {}
        };
        //println!("{:?} of {:?} is changed to {}", axis_type, gamepad, value);
      }
    }
  }
  let mut keyboard_x: f32 = 0.0;
  let mut keyboard_y: f32 = 0.0;
  if keyboard_input.pressed(KeyCode::Up) {
    keyboard_y += 1.0;
  }
  if keyboard_input.pressed(KeyCode::Down) {
    keyboard_y -= 1.0;
  }
  if keyboard_input.pressed(KeyCode::Left) {
    keyboard_x += -1.0;
  }
  if keyboard_input.pressed(KeyCode::Right) {
    keyboard_x += 1.0;
  }
  input.clock_direction = 1;
  if keyboard_input.pressed(KeyCode::X) {
    input.clock_direction = -1;
  }
  input.x = input.pad_x.clone() + keyboard_x;
  input.y = input.pad_y.clone() + keyboard_y;
}
