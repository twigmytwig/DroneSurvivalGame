use bevy::prelude::*;
use crate::ascii_sprite::AsciiSprite;
use crate::physics::Velocity;

pub struct ProjectileConfig{
    pub projectileShape: String,
    pub color: Color,
    pub font_size: f32,
    pub speed: f32,
    pub damage: u32,
}

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub struct PlayerOwned;

pub fn spawn_player_projectile(
    commands: &mut Commands,
    position: Vec2,
    direction: Vec2,
    config: &ProjectileConfig,
) {
    commands.spawn((
        Transform::from_translation(position.extend(0.0)),
        AsciiSprite {
            glyph: config.projectileShape.clone(),
            color: config.color,
            font_size: config.font_size,
            bg_color: None,
        },
        Velocity {
            direction: direction.normalize(),
            speed: config.speed,
        },
        // Damage(config.damage),  // TODO: add Damage component later
        Projectile,
        PlayerOwned, //THis makes this function only work for the player
    ));
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, _app: &mut App) {
        // Projectiles spawned via spawn_projectile()
        // Collision systems will be added here later
    }
}

impl ProjectileConfig{
    pub fn player_bullet() -> Self{
        Self { 
            projectileShape: "()".to_string(), 
            color: Color::WHITE, 
            font_size: 24.0, 
            speed: 500.0, 
            damage: 1 
        }
    }
}