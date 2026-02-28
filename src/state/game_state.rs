use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState{
    #[default]
    Loading,
    Paused,
    Playing,
    Crafting,
    GameOver,
    Victory,
}

/// Computed state that's active when the game is "in progress" (Playing, Crafting or Paused).
/// This allows SubStates like WavePhase to persist across pause/unpause.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum InGame {
    Yes,
}

impl ComputedStates for InGame {
    // The source state(s) we compute from
    type SourceStates = GameState;

    // Return Some(InGame::Yes) when active, None when inactive
    fn compute(source: GameState) -> Option<Self> {
        match source {
            GameState::Playing | GameState::Crafting | GameState::Paused => Some(InGame::Yes),
            _ => None,
        }
    }
}

// Now WavePhase sources from InGame instead of GameState::Playing.
// This means WavePhase stays active (and keeps its value) during pause!
#[derive(SubStates, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[source(InGame = InGame::Yes)]
pub enum WavePhase {
    #[default]
    Countdown,   // Brief pause before wave
    Spawning,    // Actively spawning drones
    InProgress,  // All spawned, waiting for kills
    Complete,    // All waves done (victory)
}

/// SubState that tracks which screen you're viewing while GameState::Paused is active.
/// This is NOT a replacement for Paused - it's a child state that only exists during Paused.
/// When you unpause, PauseScreen disappears. When you pause again, it resets to Main.
///
/// Navigation: Main -> Settings -> Audio (Back buttons return to previous screen)
#[derive(SubStates, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[source(GameState = GameState::Paused)]
pub enum PauseScreen {
    #[default]
    Main,      // Resume / Settings / Quit
    Settings,  // Audio / Back
    Audio,     // Volume sliders / Back
}