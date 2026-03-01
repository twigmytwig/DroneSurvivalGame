use std::time::Duration;
use bevy::prelude::*;
use super::projectile::ProjectileConfig;

/// Describes how a weapon fires its projectiles
#[derive(Clone, Copy)]
pub enum FirePattern {
    /// Single projectile straight ahead
    Single,
    /// Multiple projectiles in a spread (count, total spread angle in degrees)
    Spread { count: usize, angle_degrees: f32 },
}

/// The active weapon on an entity — built from a WeaponType
#[derive(Component)]
pub struct Weapon {
    pub weapon_type: WeaponType,
    pub config: ProjectileConfig,
    pub fire_cooldown: Timer,
    pub fire_pattern: FirePattern,
}

impl Weapon {
    pub fn from_type(weapon_type: WeaponType) -> Self {
        Self {
            weapon_type,
            config: weapon_type.projectile_config(),
            fire_cooldown: Timer::new(
                Duration::from_secs_f32(weapon_type.fire_cooldown_secs()),
                TimerMode::Repeating,
            ),
            fire_pattern: weapon_type.fire_pattern(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum WeaponType {
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

    pub fn projectile_config(&self) -> ProjectileConfig {
        match self {
            WeaponType::Pistol => ProjectileConfig::player_bullet(),
            WeaponType::Shotgun => ProjectileConfig {
                projectileShape: "o".to_string(),
                color: Color::srgb(1.0, 0.6, 0.0),
                font_size: 24.0,
                speed: 700.0,
                damage: 1,
            },
        }
    }

    pub fn fire_cooldown_secs(&self) -> f32 {
        match self {
            WeaponType::Pistol => 0.3,
            WeaponType::Shotgun => 0.6,
        }
    }

    pub fn fire_pattern(&self) -> FirePattern {
        match self {
            WeaponType::Pistol => FirePattern::Single,
            WeaponType::Shotgun => FirePattern::Spread { count: 4, angle_degrees: 30.0 },
        }
    }
}