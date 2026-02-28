use bevy::prelude::*;
use bevy::ecs::hierarchy::ChildSpawnerCommands;
use crate::combat::WeaponType;
use crate::inventory::Inventory;
use crate::player::Player;
use crate::resources::ResourceType;

pub const HOTBAR_SLOTS: usize = 10;

// =============================================================================
// MARKER COMPONENTS
// =============================================================================

/// Root container for the hotbar UI
#[derive(Component)]
pub struct Hotbar;

/// Marks a hotbar slot by index (0-9)
#[derive(Component)]
pub struct HotbarSlot(pub usize);

/// Marks the count text for a slot (update this to change displayed count)
#[derive(Component)]
pub struct HotbarCount(pub usize);

/// Marks the glyph/icon display for a slot (update this to change displayed item)
#[derive(Component)]
pub struct HotbarGlyph(pub usize);

// =============================================================================
// SPAWN / DESPAWN
// =============================================================================

pub fn spawn_hotbar(mut commands: Commands) {
    commands.spawn((
        Hotbar,
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(5.0),
            ..default()
        },
    )).with_children(|parent| {
        for i in 0..HOTBAR_SLOTS {
            spawn_hotbar_slot(parent, i);
        }
    });

    info!("Hotbar spawned");
}

fn spawn_hotbar_slot(parent: &mut ChildSpawnerCommands, slot_index: usize) {
    parent.spawn((
        HotbarSlot(slot_index),
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
            HotbarGlyph(slot_index),
            Text::new(""),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));

        // Count in top-right corner (blank, you populate it)
        slot.spawn((
            HotbarCount(slot_index),
            Text::new(""),
            TextFont {
                font_size: 12.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(2.0),
                right: Val::Px(4.0),
                ..default()
            },
        ));
    });
}

pub fn despawn_hotbar(mut commands: Commands, query: Query<Entity, With<Hotbar>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

/// A displayable item in the hotbar (resource or weapon)
enum HotbarItem {
    Resource { glyph: &'static str, color: Color, count: u32 },
    Weapon { glyph: &'static str, color: Color },
}

/// Syncs the hotbar display with the player's inventory
pub fn update_hotbar(
    player_query: Query<&Inventory, With<Player>>,
    mut glyphs: Query<(&mut Text, &mut TextColor, &HotbarGlyph)>,
    mut counts: Query<(&mut Text, &HotbarCount), Without<HotbarGlyph>>,
) {
    let Ok(inventory) = player_query.single() else { return };

    // Collect all items into a unified list
    let mut items: Vec<(&str, HotbarItem)> = Vec::new();

    // Add resources
    for (resource_type, &count) in &inventory.resource_inventory {
        items.push((
            resource_type.name(),
            HotbarItem::Resource {
                glyph: resource_type.glyph(),
                color: resource_type.color(),
                count,
            },
        ));
    }

    // Add weapons
    for weapon_type in &inventory.weapons_inventory {
        items.push((
            weapon_type.name(),
            HotbarItem::Weapon {
                glyph: weapon_type.glyph(),
                color: weapon_type.color(),
            },
        ));
    }

    // Sort by name for consistent ordering
    items.sort_by_key(|(name, _)| *name);

    // Update glyphs
    for (mut text, mut color, glyph) in &mut glyphs {
        if let Some((_, item)) = items.get(glyph.0) {
            match item {
                HotbarItem::Resource { glyph: g, color: c, .. } |
                HotbarItem::Weapon { glyph: g, color: c } => {
                    **text = (*g).into();
                    *color = TextColor(*c);
                }
            }
        } else {
            **text = String::new();
        }
    }

    // Update counts
    for (mut text, count_marker) in &mut counts {
        if let Some((_, item)) = items.get(count_marker.0) {
            match item {
                HotbarItem::Resource { count, .. } => {
                    **text = count.to_string();
                }
                HotbarItem::Weapon { .. } => {
                    **text = String::new(); // weapons don't stack
                }
            }
        } else {
            **text = String::new();
        }
    }
}
