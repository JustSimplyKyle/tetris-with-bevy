mod blocks;
use bevy::prelude::*;
use blocks::blocks::TetrisBlockPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord, Hash, States)]
pub enum GameState {
    StartMenu,
    #[default]
    InGame,
    GameOver,
}

fn start_game(
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) && state.get() == &GameState::StartMenu {
        next_state.set(GameState::InGame);
    }
}

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, start_game)
        .add_plugins(TetrisBlockPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
