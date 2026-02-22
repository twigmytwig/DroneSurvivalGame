use bevy::prelude::*;
use crate::physics::CircleHitBox;
use crate::player::Player;
use crate::ascii_sprite::AsciiSprite;
use crate::combat::{Weapon, ProjectileConfig};

pub fn test_spawn_player(
    mut commands: Commands
){
        commands.spawn((
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
        CircleHitBox{ radius: 10.0}
    ));
}