use bevy::prelude::*;
use super::projectile::ProjectileConfig;

#[derive(Component)]
pub struct Weapon {
    pub config: ProjectileConfig,
    pub fire_cooldown: Timer,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum WeaponType{
    Pistol,
    Shotgun,
}

impl WeaponType {
    pub fn name(&self) -> &'static str {
        match self {
            WeaponType::Pistol => "Pistol",
            WeaponType::Shotgun => "Shotgun",
        }
    }

    pub fn glyph(&self) -> &'static str {
        match self {
            WeaponType::Pistol => "P",
            WeaponType::Shotgun => "S",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            WeaponType::Pistol => Color::WHITE,
            WeaponType::Shotgun => Color::WHITE,
        }
    }
}