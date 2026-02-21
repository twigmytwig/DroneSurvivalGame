mod game_state;
mod loading;
mod playing;

use bevy::prelude::*;
use crate::game_fonts::{self, GameFonts};

pub use game_state::GameState;

#[derive(Resource)]
struct LoadingTimer(Timer);

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(LoadingTimer(Timer::from_seconds(1.0, TimerMode::Once)))
        .init_state::<GameState>()
        
        // Loading state systems
        .add_systems(OnEnter(GameState::Loading), (loading::spawn_loading_screen, game_fonts::load_fonts))
        .add_systems(Update, (
            check_assets_loaded,
            loading::animate_loading,
        ).run_if(in_state(GameState::Loading)))
        .add_systems(OnExit(GameState::Loading), (
            loading::despawn_loading_screen,
        ))

        //playing state systems
        .add_systems(OnEnter(GameState::Playing), playing::test_spawn_player);
    }
}


fn check_assets_loaded(
    time: Res<Time>,
    fonts: Option<Res<GameFonts>>, //we defined htis resource in game_fonts
    font_assets: Res<Assets<Font>>, //bevys internal storage for all loaded fonts
    mut next_state: ResMut<NextState<GameState>>, //bevy defined resourece
    mut timer: ResMut<LoadingTimer>,
) {
    timer.0.tick(time.delta());
    // GameFonts resource not inserted yet
    let Some(fonts) = fonts else {
        return;
    };

    // Check if the font asset has finished loading
    if font_assets.get(&fonts.mono).is_some() {
        info!("Assets loaded, transitioning to Playing!");
        if timer.0.is_finished(){
            next_state.set(GameState::Playing);
        }
    }
}