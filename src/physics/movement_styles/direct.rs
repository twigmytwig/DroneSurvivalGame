use bevy::prelude::*;
use crate::{physics::{DesiredDirection, Velocity}, state::GameState};

#[derive(Component)]
pub struct DirectMovement;

//moving to target in the shortest path (like a straight line if target is stationary)
//We need to get all entities with DirectMovement, DesiredDirection and Velocity components
//For each, we update their velocity direction to match DesiredDirection
fn move_direct(
    mut query: Query<(&DesiredDirection, &mut Velocity), With<DirectMovement>>,
) {
    for (desired, mut velocity) in &mut query {
        velocity.direction = desired.0;
    }
}

pub struct DirectMovementPlugin;

impl Plugin for DirectMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_direct.run_if(in_state(GameState::Playing)));
    }
}