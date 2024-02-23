/// This module defines the game state and related functionality.
/// The `GameState` enum represents the different states of the game,
/// including "InGame", "Paused", and "GameOver".
/// The `StatePlugin` struct is a Bevy plugin that initializes the game state
/// and adds systems for updating the state based on input events.
/// The `game_state_input_events` function handles keyboard input events
/// and transitions the game state between "InGame" and "Paused" based on the
/// "Escape" key. The `transition_to_in_game` function transitions the game state
/// to "InGame" when called.
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    InGame,
    Paused,
    GameOver,
}

pub struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_systems(
            Update,
            (
                game_state_input_events,
                transition_to_in_game.run_if(in_state(GameState::GameOver)),
            ),
        );
    }
}

fn game_state_input_events(
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::InGame => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::InGame),
            _ => (),
        }
    }
}

fn transition_to_in_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::InGame);
}
