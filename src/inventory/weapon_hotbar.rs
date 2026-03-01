use bevy::prelude::*;

use crate::{inventory::{Inventory, WEAPON_SLOTS}, player::Player};

/// Root container for the weapon hotbar UI
#[derive(Component)]
pub struct WeaponHotbar;

/// Marks a hotbar slot by index (0-2 for three weapons)
#[derive(Component)]
pub struct WeaponHotbarSlot(pub usize);

/// Marks the glyph/icon display for a slot (update this to change displayed item)
#[derive(Component)]
pub struct WeaponHotbarGlyph(pub usize);

pub fn despawn_weapon_hotbar(mut commands: Commands, query: Query<Entity, With<WeaponHotbar>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_weapon_hotbar(mut commands: Commands){
    commands.spawn((
    WeaponHotbar,
    Node {
        position_type: PositionType::Absolute,
        bottom: Val::Px(100.0),
        width: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(5.0),
        ..default()
    },
    )).with_children(|parent| {
        for i in 0..WEAPON_SLOTS {
            spawn_weapon_hotbar_slot(parent, i);
        }
    });

    info!("Weapon Hotbar spawned");
}

fn spawn_weapon_hotbar_slot(parent: &mut ChildSpawnerCommands, slot_index: usize) {
    parent.spawn((
        WeaponHotbarSlot(slot_index),
        Node {
            width: Val::Px(50.0),
            height: Val::Px(50.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.9)),
    )).with_children(|slot: &mut ChildSpawnerCommands| {
        // Glyph in the center (blank, you populate it)
        slot.spawn((
            WeaponHotbarGlyph(slot_index),
            Text::new(""),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
    });
}

pub fn update_weapon_hotbar(
    player_query: Query<&Inventory, With<Player>>,
    mut glyphs: Query<(&mut Text, &mut TextColor, &WeaponHotbarGlyph)>,
    mut slots: Query<(&mut BackgroundColor, &WeaponHotbarSlot)>,
) {
    let Ok(inventory) = player_query.single() else { return };

    // Update glyphs from weapon slots
    for (mut text, mut color, glyph) in &mut glyphs {
        if let Some(Some(weapon_type)) = inventory.weapon_slots.get(glyph.0) {
            **text = weapon_type.glyph().into();
            *color = TextColor(weapon_type.color());
        } else {
            **text = String::new();
        }
    }

    // Highlight active slot yellow, inactive gray
    for (mut bg, slot) in &mut slots {
        if slot.0 == inventory.active_weapon_slot {
            *bg = BackgroundColor(Color::srgba(0.8, 0.8, 0.0, 0.9));
        } else {
            *bg = BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.9));
        }
    }
}