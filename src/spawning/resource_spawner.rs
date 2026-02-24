use bevy::prelude::*;
use rand::RngExt;
use crate::ascii_sprite::AsciiSprite;
use crate::resources::{ResourceDrop, ResourceLifeTimer, ResourceType};

const RESOURCE_LIFETIME_SECS: f32 = 30.0;
const SPREAD_RADIUS: f32 = 20.0;

/// Spawns a resource drop at the given position with a random offset
pub fn spawn_resource(
    commands: &mut Commands,
    resource_type: ResourceType,
    base_pos: Vec2,
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
        ResourceLifeTimer(Timer::from_seconds(RESOURCE_LIFETIME_SECS, TimerMode::Once)),
    ));
}

/// Spawns multiple resources of the same type
pub fn spawn_resources(
    commands: &mut Commands,
    resource_type: ResourceType,
    base_pos: Vec2,
    count: u32,
) {
    for _ in 0..count {
        spawn_resource(commands, resource_type, base_pos);
    }
}
