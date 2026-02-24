use std::time::Duration;

use bevy::prelude::*;
use crate::physics::CircleHitBox;
use crate::player::Player;
use crate::ascii_sprite::AsciiSprite;
use crate::combat::{Health, HealthBar, ProjectileConfig, Weapon};

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        AsciiSprite {
            glyph: "@".to_string(),
            color: Color::WHITE,
            font_size: 24.0,
            bg_color: None
        },
        Player,
        Weapon {
            config: ProjectileConfig::player_bullet(),
            fire_cooldown: Timer::new(Duration::from_secs_f32(0.3) , TimerMode::Repeating)
        },
        CircleHitBox { radius: 10.0 },
        Health::new(10),
        HealthBar { max_width: 32.0, offset: 24.0 },
    ));
}