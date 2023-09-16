use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
  #[default]
  Menu,
  InGame,
}

pub fn build_app() -> App {
  let mut app = App::new();
  app.add_plugins(DefaultPlugins);
  app.add_state::<GameState>();
  app.add_systems(Update, sys.run_if(in_state(GameState::Menu)));
  app
}

fn sys(mut stat: ResMut<State<GameState>>) {
  let current = *stat.get();
  *stat = match current {
    GameState::InGame => State::new(GameState::Menu),
    GameState::Menu => State::new(GameState::InGame),
  };
  println!("{:?}", stat.get());
}

fn trans() {
  println!("Trans!")
}
