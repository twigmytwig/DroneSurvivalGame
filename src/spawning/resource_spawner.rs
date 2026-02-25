use bevy::prelude::*;
use rand::RngExt;
use crate::ascii_sprite::AsciiSprite;
use crate::physics::{DesiredDirection, MagneticAttraction, MagnetizedTo, Velocity};
use crate::resources::{ResourceDrop, ResourceLifeTimer, ResourceType};

const RESOURCE_LIFETIME_SECS: f32 = 30.0;
const SPREAD_RADIUS: f32 = 20.0;
const MAGNETIC_RANGE: f32 = 200.0;
const MAGNETIC_PEAK_SPEED: f32 = 100.0;  
const MAGNETIC_LERP_FACTOR: f32 = 0.1; 

/// Spawns a resource drop at the given position with a random offset
pub fn spawn_resource(
    commands: &mut Commands,
    resource_type: ResourceType,
    base_pos: Vec2,
    player: Entity,
) {
    let mut rng = rand::rng();

    // Random offset so items don't stack
    let offset = Vec2::new(
        rng.random_range(-SPREAD_RADIUS..SPREAD_RADIUS),
        rng.random_range(-SPREAD_RADIUS..SPREAD_RADIUS),
    );
    let pos = base_pos + offset;

    commands.spawn((
        Transform::from_translation(pos.extend(0.0)),
        Visibility::Inherited,
        AsciiSprite {
            glyph: resource_type.glyph().to_string(),
            color: resource_type.color(),
            font_size: 20.0,
            bg_color: None,
        },
        ResourceDrop { resource_type },
        MagnetizedTo{target: player, range: MAGNETIC_RANGE},
        MagneticAttraction{peak_speed: MAGNETIC_PEAK_SPEED, lerp_factor: MAGNETIC_LERP_FACTOR},
        Velocity{speed: 0.0, direction: Vec2::ZERO},
        DesiredDirection::default(),
        ResourceLifeTimer(Timer::from_seconds(RESOURCE_LIFETIME_SECS, TimerMode::Once)),
    ));
}

/// Spawns multiple resources of the same type
pub fn spawn_resources(
    commands: &mut Commands,
    resource_type: ResourceType,
    base_pos: Vec2,
    count: u32,
    player: Entity,
) {
    for _ in 0..count {
        spawn_resource(commands, resource_type, base_pos, player);
    }
}
