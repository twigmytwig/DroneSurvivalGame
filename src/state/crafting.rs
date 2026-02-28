use bevy::prelude::*;

use crate::state::GameState;

pub fn toggle_crafting(
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    input: Res<ButtonInput<KeyCode>>
){
    if input.just_pressed(KeyCode::Tab){
        match current_state.get(){
            GameState::Playing => {
                next_state.set(GameState::Crafting);
            }
            GameState::Crafting => {
                next_state.set(GameState::Playing);
            }
            _ => {}
        }
    }
}