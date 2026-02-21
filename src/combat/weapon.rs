use bevy::prelude::*;
use super::projectile::ProjectileConfig;

#[derive(Component)]
pub struct Weapon {
    pub config: ProjectileConfig,
}
