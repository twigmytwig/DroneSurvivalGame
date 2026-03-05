use bevy::prelude::*;

use crate::{ascii_sprite::AsciiSprite, combat::{Health, HealthBar}, physics::CircleHitBox, spawning::BehaviorConfig};
use super::extraction_beacon::ExtractionBeacon;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum PlaceableType{
    ExtractionBeacon,
    Turret,
    Wall
}

#[derive(Component)]
pub struct Structure;

pub fn spawn_structure(
    commands: &mut Commands,
    pos: Vec2,
    config: &PlaceableConfig,
) -> Entity {
    let mut entity = commands.spawn((
        Transform::from_translation(pos.extend(0.0)),
        AsciiSprite {
            glyph: config.glyph.to_string(),
            color: config.color,
            font_size: config.font_size,
            bg_color: None,
        },
        Structure,
    ));

    if let Some(hitbox_radius) = config.hitbox_radius {
        entity.insert(CircleHitBox { radius: hitbox_radius });
    }

    if let Some(health) = config.health {
        entity.insert(Health::new(health));
        if config.has_health_bar{
            entity.insert(HealthBar{max_width: 32.0, offset: 24.0}); //TODO: FIX HARD CODED VALUES
        }
    }

    // Attach behaviors (for turrets later - note: turrets need different targeting than drones)
    for behavior in &config.behaviors {
        match behavior {
            BehaviorConfig::ShootAtTarget { cooldown_secs, projectile } => {
                // TODO: Turrets need to find enemies dynamically, not a fixed target
                // For now, skip - will need a TurretTargeting component instead
                warn!("ShootAtTarget on structures not yet implemented");
            }
            _ => {
                // Other behaviors (CollideTarget, ExplodeOnContact, MaintainRange)
                // don't make sense for stationary structures
            }
        }
    }

    if let Some(charge_secs) = config.charge_time_secs {
        entity.insert(ExtractionBeacon::new(charge_secs));
    }

    entity.id()
}

//Todo: maybe some things have velocity?
//todo: WHAT IF some things want a health bar like turrets
pub struct PlaceableConfig {
    pub kind: PlaceableType,
    pub name: &'static str,
    pub glyph: &'static str,
    pub color: Color,
    pub font_size: f32,
    pub health: Option<u32>,           // None = invincibl
    pub has_health_bar: bool,
    pub hitbox_radius: Option<f32>,    // None = no collision
    pub behaviors: Vec<BehaviorConfig>, // empty for passive structures
    // beacon-specific
    pub charge_time_secs: Option<f32>, // only Some for beacon (for now heheheheh)
}

impl PlaceableConfig {
    pub fn from_type(placeable_type: &PlaceableType) -> Self {
        match placeable_type {
            PlaceableType::ExtractionBeacon => Self::extraction_beacon(),
            PlaceableType::Turret => todo!("Turret config"),
            PlaceableType::Wall => Self::wall(),
        }
    }

    pub fn extraction_beacon() -> Self {
        Self{
            kind: PlaceableType::ExtractionBeacon,
            name: "Extraction Beacon",
            glyph: "/*\\",
            color: Color::WHITE,
            font_size: 24.0,
            health: Some(100),
            hitbox_radius: Some(12.0),
            behaviors: vec![], 
            charge_time_secs: Some(60.0),
            has_health_bar: true,
        }
    }
    pub fn wall() -> Self {
        Self {
            kind: PlaceableType::Wall,
            name: "Wall",
            glyph: "#",
            color: Color::linear_rgb(1.0, 20.0, 100.0),
            font_size: 48.0,
            health: Some(50),
            hitbox_radius: Some(16.0),
            behaviors: vec![],
            charge_time_secs: None,
            has_health_bar: true,
        }
    }
    //pub fn turret() -> Self { ... }  // later
}