use bevy::prelude::*;

use crate::audio::{play_music, stop_music, MusicTrack};
use crate::combat::Projectile;
use crate::enemy::Enemy;
use crate::player::Player;
use crate::resources::ResourceDrop;
use crate::spawning::WaveState;
use crate::state::GameState;

#[derive(Component)]
pub struct GameOverMenu;

pub fn spawn_game_over_menu(mut commands: Commands) {
    commands.spawn((
        GameOverMenu,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
    )).with_children(|parent| {
        parent.spawn((
            Text::new("Game Over\n\nPress ESC to restart"),
            TextFont {
                font_size: 36.0,
                ..default()
            },
            TextColor(Color::WHITE),
            TextLayout::new_with_justify(Justify::Center),
        ));
    });
    
    info!("Defeat menu spawned");
}

pub fn play_game_over_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    play_music(&mut commands, &asset_server, "sounds/music/game_over_track.mp3");
}

pub fn despawn_game_over_menu(
    mut commands: Commands,
    query: Query<Entity, With<GameOverMenu>>,
    music_query: Query<Entity, With<MusicTrack>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    // Stop game over music
    for entity in &music_query {
        commands.entity(entity).despawn();
    }

    info!("Defeat menu despawned");
}

pub fn cleanup_game_entities(
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
    enemies: Query<Entity, With<Enemy>>,
    projectiles: Query<Entity, With<Projectile>>,
    resources: Query<Entity, With<ResourceDrop>>,
    mut wave_state: ResMut<WaveState>,
) {
    for entity in players.iter() {
        commands.entity(entity).despawn();
    }
    for entity in enemies.iter() {
        commands.entity(entity).despawn();
    }
    for entity in projectiles.iter() {
        commands.entity(entity).despawn();
    }
    for entity in resources.iter() {
        commands.entity(entity).despawn();
    }

    // Reset wave state for next game
    *wave_state = WaveState::default();

    info!("Game entities cleaned up");
}

//check for restart input -- refresh all data
pub fn toggle_restart(
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
){
    if input.just_pressed(KeyCode::Escape){
        next_state.set(GameState::Loading);
    }
}
