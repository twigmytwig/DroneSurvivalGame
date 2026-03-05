mod grid;
mod placeable;
mod extraction_beacon;
mod build_mode;

pub use extraction_beacon::*;
pub use placeable::*;
pub use grid::*;
pub use build_mode::*;

use bevy::prelude::*;
use crate::state::GameState;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BuildGrid>()
            .init_resource::<BuildMode>()
            .add_systems(Update, (
                build_mode::toggle_build_mode,
                build_mode::cycle_placeable,
                build_mode::manage_grid_overlay,
                build_mode::update_grid_overlay_position,
                build_mode::update_ghost_preview,
            ).run_if(in_state(GameState::Playing)))
            .add_systems(Update, (
                build_mode::place_structure,
                extraction_beacon::tick_beacon_charge,
            ).run_if(in_state(GameState::Playing)));
    }
}