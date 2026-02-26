use bevy::prelude::*;
use crate::audio::{AudioSettings, MusicTrack, play_music};

#[derive(Component)]
pub struct VictoryMenu;

pub fn spawn_victory_menu(mut commands: Commands) {
    commands.spawn((
        VictoryMenu,
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
            Text::new("Victory Over the Clankers!\n\nPress ESC to restart"),
            TextFont {
                font_size: 36.0,
                ..default()
            },
            TextColor(Color::WHITE),
            TextLayout::new_with_justify(Justify::Center),
        ));
    });
    
    info!("Victory menu spawned");
}

pub fn play_victory_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sound_setting: Res<AudioSettings>,
) {
    play_music(&mut commands, &asset_server, "sounds/music/action_man.mp3", &sound_setting);
}

pub fn despawn_victory_menu(
    mut commands: Commands,
    query: Query<Entity, With<VictoryMenu>>,
    music_query: Query<Entity, With<MusicTrack>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    // Stop victory music
    for entity in &music_query {
        commands.entity(entity).despawn();
    }

    info!("Victory menu despawned");
}