use bevy::prelude::*;
use std::collections::HashMap;
use crate::{combat::WeaponType, resources::ResourceType};

pub const WEAPON_SLOTS: usize = 3;

#[derive(Component)]
pub struct Inventory{
    pub resource_inventory: HashMap<ResourceType, u32>, //stackable
    pub weapon_slots: [Option<WeaponType>; WEAPON_SLOTS],
    pub active_weapon_slot: usize,
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

impl Default for Inventory {
    fn default() -> Self {
        Self {
            resource_inventory: HashMap::new(),
            weapon_slots: [Some(WeaponType::Pistol), None, None],
            active_weapon_slot: 0,
        }
    }
}