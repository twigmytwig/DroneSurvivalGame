use bevy::prelude::*;
use bevy::ecs::hierarchy::ChildSpawnerCommands;

use crate::{audio::AudioSettings, state::{GameState, PauseScreen}};

// =============================================================================
// MARKER COMPONENTS (for despawning each menu)
// =============================================================================

#[derive(Component)]
pub struct PauseMenu;

#[derive(Component)]
pub struct SettingsMenu;

#[derive(Component)]
pub struct AudioMenu;

// =============================================================================
// BUTTON COMPONENTS (to identify which button was clicked)
// =============================================================================

#[derive(Component)]
pub enum PauseButton {
    Resume,
    Settings,
    Quit,
}

#[derive(Component)]
pub enum SettingsButton {
    Audio,
    Back,
}

#[derive(Component)]
pub enum AudioButton {
    Back,
}

// =============================================================================
// VOLUME CONTROL COMPONENTS (for audio settings UI)
// =============================================================================

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum VolumeCategory {
    Master,
    Sfx,
    Music,
}

/// Button to adjust volume (+10% or -10%)
#[derive(Component)]
pub struct VolumeAdjustButton {
    pub category: VolumeCategory,
    pub delta: i32,  // +10 or -10
}

/// Text displaying the current volume percentage
#[derive(Component)]
pub struct VolumeValueText(pub VolumeCategory);

// =============================================================================
// MAIN PAUSE MENU (Resume / Settings / Quit)
// =============================================================================

pub fn spawn_pause_menu(mut commands: Commands) {
    commands.spawn((
        PauseMenu,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
    )).with_children(|parent| {
        // Title
        parent.spawn((
            Text::new("PAUSED"),
            TextFont { font_size: 48.0, ..default() },
            TextColor(Color::WHITE),
        ));

        // Resume button
        parent.spawn((
            Button,
            PauseButton::Resume,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
        )).with_children(|btn| {
            btn.spawn((
                Text::new("Resume"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });

        // Settings button
        parent.spawn((
            Button,
            PauseButton::Settings,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
        )).with_children(|btn| {
            btn.spawn((
                Text::new("Settings"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });

        // Quit button
        parent.spawn((
            Button,
            PauseButton::Quit,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
        )).with_children(|btn| {
            btn.spawn((
                Text::new("Quit"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });
    });
}

pub fn despawn_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn handle_pause_buttons(
    query: Query<(&Interaction, &PauseButton), Changed<Interaction>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_pause_screen: ResMut<NextState<PauseScreen>>,
    mut exit: MessageWriter<AppExit>,
) {
    for (interaction, button) in &query {
        if *interaction == Interaction::Pressed {
            match button {
                PauseButton::Resume => { next_game_state.set(GameState::Playing); }
                PauseButton::Settings => { next_pause_screen.set(PauseScreen::Settings); }
                PauseButton::Quit => { exit.write(AppExit::Success); }
            }
        }
    }
}

// =============================================================================
// SETTINGS MENU (Audio / Back)
// =============================================================================

pub fn spawn_settings_menu(mut commands: Commands) {
    commands.spawn((
        SettingsMenu,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
    )).with_children(|parent| {
        // Title
        parent.spawn((
            Text::new("SETTINGS"),
            TextFont { font_size: 48.0, ..default() },
            TextColor(Color::WHITE),
        ));

        // Audio button
        parent.spawn((
            Button,
            SettingsButton::Audio,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
        )).with_children(|btn| {
            btn.spawn((
                Text::new("Audio"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });

        // Back button
        parent.spawn((
            Button,
            SettingsButton::Back,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
        )).with_children(|btn| {
            btn.spawn((
                Text::new("Back"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });
    });
}

pub fn despawn_settings_menu(mut commands: Commands, query: Query<Entity, With<SettingsMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn handle_settings_buttons(
    query: Query<(&Interaction, &SettingsButton), Changed<Interaction>>,
    mut next_pause_screen: ResMut<NextState<PauseScreen>>,
) {
    for (interaction, button) in &query {
        if *interaction == Interaction::Pressed {
            match button {
                SettingsButton::Audio => { next_pause_screen.set(PauseScreen::Audio); }
                SettingsButton::Back => { next_pause_screen.set(PauseScreen::Main); }
            }
        }
    }
}

// =============================================================================
// AUDIO MENU (Volume sliders / Back)
// =============================================================================

pub fn spawn_audio_menu(mut commands: Commands, audio_settings: Res<AudioSettings>) {
    commands.spawn((
        AudioMenu,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
    )).with_children(|parent| {
        // Title
        parent.spawn((
            Text::new("AUDIO"),
            TextFont { font_size: 48.0, ..default() },
            TextColor(Color::WHITE),
        ));

        // Master volume control
        spawn_volume_row(parent, "Master", VolumeCategory::Master, audio_settings.master as i32);

        // SFX volume control
        spawn_volume_row(parent, "SFX", VolumeCategory::Sfx, audio_settings.sfx as i32);

        // Music volume control
        spawn_volume_row(parent, "Music", VolumeCategory::Music, audio_settings.music as i32);

        // Back button
        parent.spawn((
            Button,
            AudioButton::Back,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
        )).with_children(|btn| {
            btn.spawn((
                Text::new("Back"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });
    });
}

/// Helper to spawn a volume control row: Label [ - ] 50% [ + ]
fn spawn_volume_row(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    category: VolumeCategory,
    initial_value: i32
) {
    parent.spawn((
        Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(10.0),
            ..default()
        },
    )).with_children(|row: &mut ChildSpawnerCommands| {
        // Label
        row.spawn((
            Text::new(format!("{:8}", label)),
            TextFont { font_size: 20.0, ..default() },
            TextColor(Color::WHITE),
            Node {
                width: Val::Px(80.0),
                ..default()
            },
        ));

        // Minus button
        row.spawn((
            Button,
            VolumeAdjustButton { category, delta: -10 },
            Node {
                width: Val::Px(40.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
        )).with_children(|btn: &mut ChildSpawnerCommands| {
            btn.spawn((
                Text::new("-"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });

        // Value text
        row.spawn((
            Text::new(format!("{}%", initial_value)),
            TextFont { font_size: 20.0, ..default() },
            TextColor(Color::WHITE),
            VolumeValueText(category),
            Node {
                width: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
        ));

        // Plus button
        row.spawn((
            Button,
            VolumeAdjustButton { category, delta: 10 },
            Node {
                width: Val::Px(40.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
        )).with_children(|btn: &mut ChildSpawnerCommands| {
            btn.spawn((
                Text::new("+"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });
    });
}

pub fn despawn_audio_menu(mut commands: Commands, query: Query<Entity, With<AudioMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

//the menu buttons, not the actual audio setting changers
pub fn handle_audio_buttons(
    query: Query<(&Interaction, &AudioButton), Changed<Interaction>>,
    mut next_pause_screen: ResMut<NextState<PauseScreen>>,
) {
    for (interaction, button) in &query {
        if *interaction == Interaction::Pressed {
            match button {
                AudioButton::Back => { next_pause_screen.set(PauseScreen::Settings); }
            }
        }
    }
}

pub fn handle_volume_buttons(
    volume_button_query: Query<(&Interaction, &VolumeAdjustButton), Changed<Interaction>>,
    mut audio_settings: ResMut<AudioSettings>,
    mut text_query: Query<(&mut Text, &VolumeValueText)>,
) {
    for (interaction, volume_button) in &volume_button_query {
        if *interaction == Interaction::Pressed {
            // Update the setting
            let new_value = match volume_button.category {
                VolumeCategory::Master => {
                    audio_settings.master = (audio_settings.master + volume_button.delta as f32).clamp(0.0, 100.0);
                    audio_settings.master
                }
                VolumeCategory::Music => {
                    audio_settings.music = (audio_settings.music + volume_button.delta as f32).clamp(0.0, 100.0);
                    audio_settings.music
                }
                VolumeCategory::Sfx => {
                    audio_settings.sfx = (audio_settings.sfx + volume_button.delta as f32).clamp(0.0, 100.0);
                    audio_settings.sfx
                }
            };

            // Update the display text
            for (mut text, value_text) in &mut text_query {
                if value_text.0 == volume_button.category {
                    **text = format!("{}%", new_value as i32);
                }
            }
        }
    }
}
