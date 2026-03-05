use bevy::prelude::*;
use crate::building::PlaceableType;
use crate::inventory::{Inventory, add_placeable};
use crate::physics::CircleHitBox;
use crate::player::Player;
use crate::ascii_sprite::AsciiSprite;
use crate::combat::{Health, HealthBar, Weapon, WeaponType};

pub fn spawn_player(
    mut commands: Commands,
    players: Query<Entity, With<Player>>
) {
    if players.count() == 0{
        let mut inventory = Inventory::default();
        // TODO: remove test items
        add_placeable(&mut inventory, PlaceableType::ExtractionBeacon, 1);
        add_placeable(&mut inventory, PlaceableType::Wall, 1);

        commands.spawn((
            Transform::from_xyz(0.0, 0.0, 0.0),
            AsciiSprite {
                glyph: "@".to_string(),
                color: Color::WHITE,
                font_size: 24.0,
                bg_color: None
            },
            Player,
            Weapon::from_type(WeaponType::Pistol),
            CircleHitBox { radius: 10.0 },
            Health::new(10),
            HealthBar { max_width: 32.0, offset: 24.0 },
            inventory,
        ));
    }

}