// physics.rs
mod velocity;
mod hitbox;
mod movement_styles;

pub use velocity::*;
pub use hitbox::*;
pub use movement_styles::*;

use bevy::prelude::*;

/// Set by targeting behaviors (FollowTarget, MaintainRange, etc.)
/// Read by movement styles (DirectMovement, ZigZag, etc.) to set Velocity
#[derive(Component, Default)]
pub struct DesiredDirection(pub Vec2);

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(velocity::VelocityPlugin)
            .add_plugins(movement_styles::MovementStylesPlugin)
            .add_plugins(hitbox::HitboxPlugin);
    }
}