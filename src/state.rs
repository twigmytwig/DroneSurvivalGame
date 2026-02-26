mod game_state;
mod loading;
mod playing;
mod game_over;
mod victory;
mod paused;

use bevy::prelude::*;
use bevy::time::common_conditions::paused;
use crate::game_fonts::{self, GameFonts};
use crate::spawning::{
    countdown_system, spawn_system, check_wave_clear,
    WaveState, WaveDefinitions,
};

pub use game_over::toggle_restart;
pub use game_state::GameState;
pub use game_state::InGame;
pub use game_state::WavePhase;

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

        //paused state systems
        .add_systems(OnEnter(GameState::Paused), paused::spawn_pause_menu)
        .add_systems(OnExit(GameState::Paused), paused::despawn_pause_menu)
        .add_systems(Update, paused::handle_button_clicks)

        //playing state systems
        .add_systems(OnEnter(GameState::Playing), playing::spawn_player)
        .add_systems(Update, toggle_pause)

        //Game Over systems
        .add_systems(OnEnter(GameState::GameOver), (
            game_over::cleanup_game_entities,
            game_over::spawn_game_over_menu,
            game_over::play_game_over_music,
        ))
        .add_systems(OnExit(GameState::GameOver), game_over::despawn_game_over_menu)
        .add_systems(Update, toggle_restart.run_if(in_state(GameState::GameOver)))

        //Victory state systems
        .add_systems(OnEnter(GameState::Victory), (
            game_over::cleanup_game_entities,//reusing this from game over
            victory::spawn_victory_menu,
            victory::play_victory_music,
        ))
        .add_systems(OnExit(GameState::Victory), victory::despawn_victory_menu)
        .add_systems(Update, toggle_restart.run_if(in_state(GameState::Victory)))

        // InGame computed state (active during Playing OR Paused)
        // Must be registered before WavePhase since WavePhase depends on it
        .add_computed_state::<game_state::InGame>()

        // Wave SubState (now sources from InGame, so it persists across pause)
        .add_sub_state::<WavePhase>()
        .init_resource::<WaveState>()
        .init_resource::<WaveDefinitions>()
        .add_systems(Update, countdown_system.run_if(in_state(WavePhase::Countdown)))
        .add_systems(Update, spawn_system.run_if(in_state(WavePhase::Spawning)))
        .add_systems(Update, check_wave_clear.run_if(in_state(WavePhase::InProgress)));
    }
}


fn check_assets_loaded(
    time: Res<Time>,
    fonts: Option<Res<GameFonts>>, //we defined htis resource in game_fonts
    font_assets: Res<Assets<Font>>, //bevys internal storage for all loaded fonts
    mut next_state: ResMut<NextState<GameState>>,
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

fn toggle_pause( // ALSO HANDLES RESTARTING GAME TODO: DONT PUT RESTART LOGIC IN HERE
    input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::Playing => {
                info!("Game paused");
                next_state.set(GameState::Paused);
            }
            GameState::Paused => {
                info!("Game resumed");
                next_state.set(GameState::Playing);
            }
            _ => {}
        }
    }
}