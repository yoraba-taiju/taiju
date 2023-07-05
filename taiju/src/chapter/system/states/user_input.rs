use bevy::{
  input::{
    gamepad::{GamepadEvent, GamepadEventType},
  },
  prelude::*,
};

use crate::chapter::states::UserInput;

pub fn update(
  mut input: ResMut<UserInput>,
  keyboard_input: Res<Input<KeyCode>>,
  mut gamepad_event: EventReader<GamepadEvent>,
) {
  println!("Input Updated");
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
      GamepadEvent(_gamepad, GamepadEventType::AxisChanged(axis_type, value)) => {
        match axis_type {
          &GamepadAxisType::DPadX => {
            input.pad_x = *value;
          },
          &GamepadAxisType::DPadY=> {
            input.pad_y = *value;
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
  input.x = input.pad_x + keyboard_x;
  input.y = input.pad_y + keyboard_y;
}
