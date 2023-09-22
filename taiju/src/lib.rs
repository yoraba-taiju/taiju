use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
  #[default]
  Menu,
  InGame,
  Exit,
}

pub fn build_app() -> App {
  let mut app = App::new();
  app.add_plugins(DefaultPlugins);
  app.add_state::<GameState>();
  app.add_systems(Update, sys.run_if(in_state(GameState::Menu)));
  app.add_systems(Update, exit.run_if(in_state(GameState::InGame)));
  app
}

fn sys(mut stat: ResMut<State<GameState>>) {
  let current = *stat.get();
  *stat = State::new(match current {
    GameState::InGame => GameState::Menu,
    GameState::Menu => GameState::InGame,
    GameState::Exit => GameState::Exit,
  });
  println!("{:?} => {:?}", current, stat.get());
}

fn exit(mut stat: ResMut<State<GameState>>) {
  println!("Exit!");
  *stat = State::new(GameState::Exit);
}
