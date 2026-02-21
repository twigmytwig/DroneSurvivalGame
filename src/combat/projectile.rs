use bevy::prelude::*;
use crate::ascii_sprite::AsciiSprite;
use crate::physics::{CircleHitBox, Velocity};

#[derive(Component)]
pub struct Lifetimer(pub Timer);

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
        Lifetimer(Timer::from_seconds(2.0, TimerMode::Once)),
        CircleHitBox,
    ));
}

fn tick_lifetimes(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Lifetimer)>,
){
    for (entity, mut lifetime) in &mut query{
        lifetime.0.tick(time.delta());
        if lifetime.0.is_finished(){
            commands.entity(entity).despawn();
        }
    }
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        // Projectiles spawned via spawn_projectile()
        // Collision systems will be added here later
        app.add_systems(Update, tick_lifetimes);
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