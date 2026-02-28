use bevy::{platform::collections::HashSet, prelude::*};
use std::collections::HashMap;
use crate::{combat::WeaponType, resources::ResourceType};

#[derive(Component, Default)]
pub struct Inventory{
    pub resource_inventory: HashMap<ResourceType, u32>, //stackable
    pub weapons_inventory: HashSet<WeaponType>,
    //weapon inventory or anything else would be a separate hashmap
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
    if inventory.weapons_inventory.is_empty() {
        info!("WeaponsInventory is empty");
        return;
    }

    for weapon_type in inventory.weapons_inventory.iter() {
        info!("{},", weapon_type.name());
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

/// Add a weapon to the inventory. Returns false if already owned.
pub fn add_weapon(inventory: &mut Inventory, weapon: WeaponType) -> bool {
    inventory.weapons_inventory.insert(weapon)
}