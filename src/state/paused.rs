use bevy::prelude::*;
use bevy::ecs::hierarchy::ChildSpawnerCommands;

use crate::state::{GameState, PauseScreen};

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
// SLIDER COMPONENTS (for audio settings UI)
// =============================================================================

#[derive(Component, Clone, Copy)]
pub enum VolumeSlider {
    Master,
    Sfx,
    Music,
}

#[derive(Component)]
pub struct SliderBar;

#[derive(Component)]
pub struct SliderFill;

#[derive(Component)]
pub struct SliderValue;

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

pub fn spawn_audio_menu(mut commands: Commands) {
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

        // Master volume slider
        spawn_slider_row(parent, "Master", VolumeSlider::Master, 0.5);

        // SFX volume slider
        spawn_slider_row(parent, "SFX", VolumeSlider::Sfx, 0.7);

        // Music volume slider
        spawn_slider_row(parent, "Music", VolumeSlider::Music, 0.4);

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

/// Helper to spawn a slider row - called from within with_children closure
fn spawn_slider_row(
    parent: &mut ChildSpawnerCommands, 
    label: &str, 
    slider_type: VolumeSlider, 
    initial_value: f32
) {
    parent.spawn((
        Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(15.0),
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

        // Slider bar (background + fill)
        row.spawn((
            Button,  // Makes it clickable
            SliderBar,
            slider_type,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        )).with_children(|bar: &mut ChildSpawnerCommands| {
            // Fill (width based on value)
            bar.spawn((
                SliderFill,
                slider_type,
                Node {
                    width: Val::Percent(initial_value * 100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.4, 0.7, 0.4)),
            ));
        });

        // Value text
        row.spawn((
            Text::new(format!("{}%", (initial_value * 100.0) as i32)),
            TextFont { font_size: 20.0, ..default() },
            TextColor(Color::WHITE),
            SliderValue,
            slider_type,
            Node {
                width: Val::Px(50.0),
                ..default()
            },
        ));
    });
}

pub fn despawn_audio_menu(mut commands: Commands, query: Query<Entity, With<AudioMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

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
