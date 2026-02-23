pub mod projectile;
pub mod weapon;
pub mod collision;
pub mod damage;
pub mod health;

pub use collision::*;
pub use projectile::*;
pub use weapon::*;
pub use damage::*;
pub use health::*;

use bevy::prelude::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(projectile::ProjectilePlugin)
            .add_plugins(collision::CollisionPlugin)
            .add_plugins(damage::DamagePlugin)
            .add_plugins(health::HealthPlugin);
    }
} 