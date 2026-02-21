pub mod projectile;
pub mod weapon;

pub use projectile::*;
pub use weapon::*;

use bevy::prelude::*;

pub struct CombatPlugin;

  impl Plugin for CombatPlugin {
      fn build(&self, app: &mut App) {
          app.add_plugins(projectile::ProjectilePlugin);
          // .add_plugins(collision::CollisionPlugin)
          // .add_plugins(weapons::WeaponsPlugin)
      }
  } 