use bevy::prelude::*;
use std::collections::HashMap;
use crate::{building::PlaceableType, combat::WeaponType, resources::ResourceType};

pub const WEAPON_SLOTS: usize = 3;

#[derive(Component)]
pub struct Inventory{
    pub resource_inventory: HashMap<ResourceType, u32>, //stackable
    pub weapon_slots: [Option<WeaponType>; WEAPON_SLOTS],
    pub active_weapon_slot: usize,
    pub placeable_inventory: HashMap<PlaceableType, u32>, //stackable
}

pub fn read_resource_inventory(inventory: &Inventory){
    if inventory.resource_inventory.is_empty() {
        info!("ResourceInventory is empty");
        return;
    }

    for (resource_type, count) in inventory.resource_inventory.iter() {
        info!("{}: {}", resource_type.name(), count);
    }
}

pub fn read_weapon_inventory(inventory: &Inventory) {
    for (i, slot) in inventory.weapon_slots.iter().enumerate() {
        match slot {
            Some(weapon_type) => info!("Slot {}: {}", i + 1, weapon_type.name()),
            None => info!("Slot {}: Empty", i + 1),
        }
    }
}



pub fn add_resource(
    inventory: &mut Inventory,
    resource: ResourceType,
    count: u32,
){
    *inventory.resource_inventory.entry(resource).or_insert(0) += count;
}

pub fn remove_resource(
    inventory: &mut Inventory,
    resource: ResourceType,
    count: u32,
){
    if let Some(cur_count) = inventory.resource_inventory.get_mut(&resource) {
        *cur_count = cur_count.saturating_sub(count);
    }
}

/// Check if inventory has enough of all the specified resources
pub fn has_resources(inventory: &Inventory, requirements: &[(ResourceType, u32)]) -> bool {
    for (resource, required) in requirements {
        let have = inventory.resource_inventory.get(resource).copied().unwrap_or(0);
        if have < *required {
            return false;
        }
    }
    true
}

/// Add a weapon to the first empty slot. Returns the slot index, or None if full.
pub fn add_weapon(inventory: &mut Inventory, weapon: WeaponType) -> Option<usize> {
    // Don't add duplicates
    if inventory.weapon_slots.iter().any(|s| *s == Some(weapon)) {
        return None;
    }
    // Find first empty slot
    for (i, slot) in inventory.weapon_slots.iter_mut().enumerate() {
        if slot.is_none() {
            *slot = Some(weapon);
            return Some(i);
        }
    }
    None
}

/// Get the weapon in a specific slot
pub fn weapon_at_slot(inventory: &Inventory, slot: usize) -> Option<WeaponType> {
    inventory.weapon_slots.get(slot).copied().flatten()
}

/// Get the currently active weapon
pub fn active_weapon(inventory: &Inventory) -> Option<WeaponType> {
    weapon_at_slot(inventory, inventory.active_weapon_slot)
}

pub fn add_placeable(inventory: &mut Inventory, placeable: PlaceableType, count: u32) {
    *inventory.placeable_inventory.entry(placeable).or_insert(0) += count;
}

pub fn remove_placeable(inventory: &mut Inventory, placeable: PlaceableType) -> bool {
    if let Some(cur_count) = inventory.placeable_inventory.get_mut(&placeable) {
        if *cur_count > 0 {
            *cur_count -= 1;
            if *cur_count == 0 {
                inventory.placeable_inventory.remove(&placeable);
            }
            return true;
        }
    }
    false
}

pub fn has_placeable(inventory: &Inventory, placeable: PlaceableType) -> bool {
    inventory.placeable_inventory.get(&placeable).copied().unwrap_or(0) > 0
}

/// Returns the first PlaceableType the player has in inventory, if any
pub fn first_available_placeable(inventory: &Inventory) -> Option<PlaceableType> {
    inventory.placeable_inventory.iter()
        .find(|(_, count)| **count > 0)
        .map(|(kind, _)| *kind)
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            resource_inventory: HashMap::new(),
            weapon_slots: [Some(WeaponType::Pistol), None, None],
            active_weapon_slot: 0,
            placeable_inventory: HashMap::new(),
        }
    }
}