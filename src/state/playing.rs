use bevy::prelude::*;
use crate::physics::{CircleHitBox, DesiredDirection, Velocity, DirectMovement};
use crate::player::Player;
use crate::ascii_sprite::AsciiSprite;
use crate::enemy::Enemy;
use crate::combat::{Health, HealthBar, ProjectileConfig, Weapon};
use crate::npc_behaviors::{CollideTarget, ExplodeOnContact, MaintainRangeFromTarget, ShootAtTarget};

pub fn test_spawn_player(
    mut commands: Commands
){
    // Spawn player first, get the Entity
    let player_entity = commands.spawn((
        Transform::from_xyz(15.0, 100.0, 0.0),
        AsciiSprite {
            glyph: "@".to_string(),
            color: Color::WHITE,
            font_size: 24.0,
            bg_color: None
        },
        Player,
        Weapon {
            config: ProjectileConfig::player_bullet(),
        },
        CircleHitBox { radius: 10.0 },
        Health::new(10),
        HealthBar{max_width:32.0, offset:24.0},
    )).id();

    // Spawn enemy that chases player
    commands.spawn((
        Transform::from_xyz(300.0, 200.0, 0.0),  // spawn away from player
        AsciiSprite {
            glyph: "<{=}>".to_string(),
            color: Color::srgb(1.0, 0.0, 0.0),  // red enemy
            font_size: 24.0,
            bg_color: None
        },
        Enemy,
        Health::new(10),
        CircleHitBox { radius: 30.0 },
        // Behavior: collide with player
        CollideTarget { target: player_entity },
        // Movement style: direct path
        DirectMovement,
        // Required for movement
        DesiredDirection::default(),
        HealthBar{max_width:32.0, offset:24.0},
        ExplodeOnContact,
        Velocity { speed: 150.0, direction: Vec2::ZERO },
    ));

    commands.spawn((
        Transform::from_xyz(100.0, -100.0, 0.0),  // spawn away from player
        AsciiSprite {
            glyph: "<{=}>".to_string(),
            color: Color::srgb(1.0, 0.0, 0.0),  // red enemy
            font_size: 24.0,
            bg_color: None
        },
        Enemy,
        Health::new(10),
        CircleHitBox { radius: 30.0 },
        // Behavior: collide with player
        MaintainRangeFromTarget{target: player_entity, range: 100.0},
        // Movement style: direct path
        DirectMovement,
        // Required for movement
        DesiredDirection::default(),
        HealthBar{max_width:32.0, offset:24.0},
        Velocity { speed: 150.0, direction: Vec2::ZERO },
        ShootAtTarget {
          target: player_entity,
          cooldown: Timer::from_seconds(0.5, TimerMode::Repeating),
          config: ProjectileConfig::enemy_bullet(),
      },
    ));
}