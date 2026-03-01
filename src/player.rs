pub mod movement;
pub mod shoot;
pub mod interact;
pub mod weapon_switch;

use bevy::prelude::*;
use crate::state::GameState;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            movement::move_player.run_if(in_state(GameState::Playing)),
            shoot::player_shoot.run_if(in_state(GameState::Playing)),
            interact::player_interact.run_if(in_state(GameState::Playing)),
            weapon_switch::weapon_switch.run_if(in_state(GameState::Playing)),
        ));
    }
}
