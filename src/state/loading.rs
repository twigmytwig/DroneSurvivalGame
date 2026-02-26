use bevy::prelude::*;
use crate::{audio::GameAudios, game_fonts::GameFonts, state::{GameState, LoadingTimer}};

#[derive(Component)]
pub struct LoadingScreen;

#[derive(Component)]
pub struct LoadingText;

pub fn spawn_loading_screen(mut commands: Commands) {
    commands.spawn((
        LoadingScreen,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
    )).with_children(|parent| {
        parent.spawn((
            LoadingText,
            Text::new("Loading..."),
            TextFont {
                font_size: 48.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
    });
    
    info!("Loading screen spawned");
}

pub fn animate_loading(
    time: Res<Time>,
    mut query: Query<&mut Text, With<LoadingText>>,
) {
    for mut text in query.iter_mut() {
        let dots = (time.elapsed_secs() * 2.0) as usize % 4;
        **text = format!("Loading{}", ".".repeat(dots));
    }
}

pub fn check_assets_loaded(
    time: Res<Time>,
    fonts: Option<Res<GameFonts>>,
    audios: Option<Res<GameAudios>>,
    font_assets: Res<Assets<Font>>,
    audio_assets: Res<Assets<AudioSource>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut timer: ResMut<LoadingTimer>,
) {
    timer.0.tick(time.delta());

    // Resources not inserted yet
    let (Some(fonts), Some(audios)) = (fonts, audios) else {
        return;
    };

    // Check if all assets have finished loading
    let fonts_loaded = font_assets.get(&fonts.mono).is_some();
    let audio_loaded = audio_assets.get(&audios.player_shoot).is_some()
        && audio_assets.get(&audios.character_hit).is_some()
        && audio_assets.get(&audios.player_hit_explosion).is_some()
        && audio_assets.get(&audios.game_over_music).is_some()
        && audio_assets.get(&audios.victory_music).is_some();

    if fonts_loaded && audio_loaded && timer.0.is_finished() {
        info!("Assets loaded, transitioning to Playing!");
        next_state.set(GameState::Playing);
    }
}

pub fn despawn_loading_screen(
    mut commands: Commands,
    query: Query<Entity, With<LoadingScreen>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    
    info!("Loading screen despawned");
}