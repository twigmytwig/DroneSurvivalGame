use bevy::prelude::*;
use crate::physics::{CircleHitBox, Velocity};
use super::damage::ProjectileDamage;

#[derive(Component)]
pub struct Lifetimer(pub Timer);

#[derive(Clone)]
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

#[derive(Component)]
pub struct EnemyOwned;

pub fn spawn_player_projectile(
    commands: &mut Commands,
    position: Vec2,
    direction: Vec2,
    config: &ProjectileConfig,
) {
    commands.spawn((
        Transform::from_translation(position.extend(0.0)),
        Sprite { //hard coded square for now
          color: config.color,
          custom_size: Some(Vec2::new(4.0, 4.0)),
          ..default()
        },
        Velocity {
            direction: direction.normalize(),
            speed: config.speed,
        },
        ProjectileDamage(config.damage),
        Projectile,
        PlayerOwned, //THis makes this function only work for the player
        Lifetimer(Timer::from_seconds(2.0, TimerMode::Once)),
        CircleHitBox{ radius: 2.0},
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

pub fn spawn_enemy_projectile(
    commands: &mut Commands,
    position: Vec2,
    direction: Vec2,
    config: &ProjectileConfig,
) {
    commands.spawn((
        Transform::from_translation(position.extend(0.0)),
        Sprite {
            color: config.color,
            custom_size: Some(Vec2::new(4.0, 4.0)),
            ..default()
        },
        Velocity {
            direction: direction.normalize(),
            speed: config.speed,
        },
        ProjectileDamage(config.damage),
        Projectile,
        EnemyOwned,
        Lifetimer(Timer::from_seconds(2.0, TimerMode::Once)),
        CircleHitBox { radius: 2.0 },
    ));
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        // Projectiles spawned via spawn_projectile()
        // Collision systems will be added here later
        app.add_systems(Update, tick_lifetimes);
    }
}

impl ProjectileConfig {
    pub fn player_bullet() -> Self {
        Self {
            projectileShape: "()".to_string(),
            color: Color::WHITE,
            font_size: 24.0,
            speed: 1000.0,
            damage: 1,
        }
    }

    pub fn enemy_bullet() -> Self {
        Self {
            projectileShape: "o".to_string(),
            color: Color::srgb(1.0, 0.0, 0.0),
            font_size: 24.0,
            speed: 400.0,
            damage: 1,
        }
    }
}