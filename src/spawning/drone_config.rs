use bevy::prelude::*;
use crate::combat::ProjectileConfig;

/// Configuration data for spawning a drone (not a component itself)
#[derive(Clone)]
pub struct DroneConfig {
    pub name: String,
    pub font_size: f32,
    pub glyph: String,
    pub color: Color,
    pub health: u32,
    pub health_bar_width: f32,
    pub health_bar_offset: f32,
    pub hitbox_radius: f32,
    pub speed: f32,
    pub behaviors: Vec<BehaviorConfig>,
    pub movement: MovementConfig,
}

/// Individual behaviors that can be mixed and matched
#[derive(Clone)]
pub enum BehaviorConfig {
    CollideTarget,
    MaintainRange { range: f32 },
    ShootAtTarget { cooldown_secs: f32, projectile: ProjectileConfig },
    ExplodeOnContact { damage: u32 },
}

#[derive(Clone)]
pub enum MovementConfig {
    Direct,
    ZigZag { amplitude: f32, frequency: f32 }, // PLANNED
}

// Preset drone configurations
impl DroneConfig {
    /// Basic chaser - runs at player and explodes
    pub fn chaser() -> Self {
        Self {
            name: "chaser".to_string(),
            glyph: "<{=}>".to_string(),
            color: Color::srgb(1.0, 0.0, 0.0),
            health: 10,
            hitbox_radius: 30.0,
            speed: 150.0,
            font_size: 24.0,
            movement: MovementConfig::Direct,
            behaviors: vec![
                BehaviorConfig::CollideTarget,
                BehaviorConfig::ExplodeOnContact { damage: 5 },
            ],
            health_bar_width: 32.0,
            health_bar_offset: 24.0
        }
    }

    /// Shooter - keeps distance and fires
    pub fn shooter() -> Self {
        Self {
            name: "shooter".to_string(),
            glyph: "=(+)=".to_string(),
            color: Color::srgb(1.0, 0.5, 0.0),
            health: 10,
            font_size: 24.0,
            hitbox_radius: 30.0,
            speed: 90.0,
            movement: MovementConfig::Direct,
            behaviors: vec![
                BehaviorConfig::MaintainRange { range: 150.0 },
                BehaviorConfig::ShootAtTarget {
                    cooldown_secs: 0.5,
                    projectile: ProjectileConfig::enemy_bullet(),
                },
            ],
            health_bar_width: 32.0,
            health_bar_offset: 24.0
        }
    }
}