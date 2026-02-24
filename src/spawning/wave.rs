use bevy::prelude::*;
use crate::{enemy::Enemy, state::GameState};
use crate::player::Player;
use crate::state::WavePhase;
use super::{spawn_drone, DroneConfig};

#[derive(Resource)]
pub struct WaveState {
    pub wave_number: usize,
    pub spawn_index: usize,
    pub spawn_timer: Timer,
    pub countdown_timer: Timer,
}

impl Default for WaveState {
    fn default() -> Self {
        Self {
            wave_number: 0,
            spawn_index: 0,
            spawn_timer: Timer::from_seconds(0.3, TimerMode::Repeating),
            countdown_timer: Timer::from_seconds(2.0, TimerMode::Once),
        }
    }
}

#[derive(Resource)]
pub struct WaveDefinitions {
    pub waves: Vec<WaveDefinition>,
}

pub struct WaveDefinition {
    pub drones: Vec<DroneConfig>,
}

impl Default for WaveDefinitions {
    fn default() -> Self {
        Self {
            waves: vec![
                // Wave 1: 3 chasers
                WaveDefinition {
                    drones: vec![
                        DroneConfig::chaser(),
                        DroneConfig::chaser(),
                        DroneConfig::chaser(),
                    ],
                },
                // Wave 2: 2 chasers + 2 shooters
                WaveDefinition {
                    drones: vec![
                        DroneConfig::chaser(),
                        DroneConfig::chaser(),
                        DroneConfig::shooter(),
                        DroneConfig::shooter(),
                    ],
                },
            ],
        }
    }
}

/// Countdown before wave starts
pub fn countdown_system(
    time: Res<Time>,
    mut wave_state: ResMut<WaveState>,
    mut next_phase: ResMut<NextState<WavePhase>>,
) {
    wave_state.countdown_timer.tick(time.delta());

    if wave_state.countdown_timer.just_finished() {
        info!("Wave {} starting!", wave_state.wave_number + 1);
        next_phase.set(WavePhase::Spawning);
    }
}

/// Spawn drones one at a time with a short delay
pub fn spawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut wave_state: ResMut<WaveState>,
    wave_defs: Res<WaveDefinitions>,
    player_query: Query<Entity, With<Player>>,
    mut next_phase: ResMut<NextState<WavePhase>>,
) {
    let Ok(player_entity) = player_query.single() else {
        return;
    };

    let Some(current_wave) = wave_defs.waves.get(wave_state.wave_number) else {
        // No more waves
        next_phase.set(WavePhase::Complete);
        return;
    };

    wave_state.spawn_timer.tick(time.delta());

    if wave_state.spawn_timer.just_finished() {
        if wave_state.spawn_index < current_wave.drones.len() {
            let config = &current_wave.drones[wave_state.spawn_index];

            // Spawn at random position around player
            let angle = wave_state.spawn_index as f32 * std::f32::consts::TAU / current_wave.drones.len() as f32;
            let spawn_distance = 400.0;
            let spawn_pos = Vec2::new(angle.cos() * spawn_distance, angle.sin() * spawn_distance);

            spawn_drone(&mut commands, config, spawn_pos, player_entity);

            info!("Spawned {} ({}/{})", config.name, wave_state.spawn_index + 1, current_wave.drones.len());
            wave_state.spawn_index += 1;
        } else {
            // All drones spawned for this wave
            info!("All drones spawned, entering InProgress");
            next_phase.set(WavePhase::InProgress);
        }
    }
}

/// Check if all enemies are dead
pub fn check_wave_clear(
    enemy_query: Query<Entity, With<Enemy>>,
    mut wave_state: ResMut<WaveState>,
    wave_defs: Res<WaveDefinitions>,
    mut next_phase: ResMut<NextState<WavePhase>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if enemy_query.iter().count() == 0 {
        info!("Wave {} cleared!", wave_state.wave_number + 1);

        wave_state.wave_number += 1;
        wave_state.spawn_index = 0;
        wave_state.countdown_timer.reset();

        if wave_state.wave_number >= wave_defs.waves.len() {
            info!("All waves complete! Victory!");
            next_phase.set(WavePhase::Complete);
            next_state.set(GameState::Victory);
        } else {
            info!("Starting countdown for wave {}", wave_state.wave_number + 1);
            next_phase.set(WavePhase::Countdown);
        }
    }
}
