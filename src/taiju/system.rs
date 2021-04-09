use bevy::{
  input::{
    gamepad::{GamepadEvent, GamepadEventType},
    keyboard::KeyboardInput,
  },
  prelude::*,
};

pub fn gamepad_events(mut gamepad_event: EventReader<GamepadEvent>) {
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
        println!("{:?} of {:?} is changed to {}", axis_type, gamepad, value);
      }
    }
  }
}

pub fn keyboard_events(keyboard_input: Res<Input<KeyCode>>) {
  if keyboard_input.just_pressed(KeyCode::A) {
    println!("'A' just pressed");
  }

  if keyboard_input.just_released(KeyCode::A) {
    println!("'A' just released");
  }
}
