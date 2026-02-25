//this system applies a magnetizing effect.
//It should take in a target, a setting for fade in/out of magnetism, and speed at peak
//IDEA: What if we just lerp the beginning and lerp the end?
use bevy::prelude::*;
use crate::{helpers::lerp, physics::{DesiredDirection, Velocity}, state::GameState};

/// Marks an entity as magnetized to a target within a range
#[derive(Component)]
pub struct MagnetizedTo {
    pub target: Entity,
    pub range: f32,
}

/// Controls the movement behavior when magnetized
#[derive(Component)]
pub struct MagneticAttraction {
    pub peak_speed: f32,
    pub lerp_factor: f32,
}

/// Updates DesiredDirection when target is within range
fn magnetized_to_system(
    targets: Query<&Transform>,
    mut magnetized: Query<(&MagnetizedTo, &Transform, &mut DesiredDirection)>,
) {
    for (magnet, transform, mut desired) in &mut magnetized {
        if let Ok(target_transform) = targets.get(magnet.target) {
            let to_target = target_transform.translation - transform.translation;
            let distance = to_target.truncate().length();

            if distance <= magnet.range {
                // Within range - point toward target
                desired.0 = to_target.truncate().normalize_or_zero();
            } else {
                // Outside range - no movement
                desired.0 = Vec2::ZERO;
            }
        }
    }
}

/// Lerps velocity toward desired direction and ramps up speed
fn magnetic_attraction_system(
    mut query: Query<(&DesiredDirection, &mut Velocity, &MagneticAttraction)>,
) {
    for (direction, mut velocity, attraction) in &mut query {
        // Lerp direction
        velocity.direction.x = lerp(velocity.direction.x, direction.0.x, attraction.lerp_factor);
        velocity.direction.y = lerp(velocity.direction.y, direction.0.y, attraction.lerp_factor);

        // Lerp speed toward peak when moving, toward 0 when stationary
        let target_speed = if direction.0 != Vec2::ZERO {
            attraction.peak_speed
        } else {
            0.0
        };
        velocity.speed = lerp(velocity.speed, target_speed, attraction.lerp_factor);
    }
}

pub struct MagnetismPlugin;

impl Plugin for MagnetismPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (magnetized_to_system, magnetic_attraction_system).chain().run_if(in_state(GameState::Playing)));
    }
}