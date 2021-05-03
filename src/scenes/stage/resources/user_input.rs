use bevy::{
  input::{
    gamepad::{GamepadEvent, GamepadEventType},
  },
  prelude::*,
};
use crate::scenes::stage::prelude::*;

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
  input.x = input.pad_x.clone() + x;
  input.y = input.pad_y.clone() + y;
}
