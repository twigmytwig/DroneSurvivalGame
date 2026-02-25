use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState{
    #[default]
    Loading,
    Paused,
    Playing,
    GameOver,
    Victory,
}

#[derive(SubStates, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[source(GameState = GameState::Playing)]
pub enum WavePhase {
    #[default]
    Countdown,   // Brief pause before wave
    Spawning,    // Actively spawning drones
    InProgress,  // All spawned, waiting for kills
    Complete,    // All waves done (victory)
}