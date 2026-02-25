use bevy::prelude::*;
use std::collections::HashMap;
use crate::resources::ResourceType;

#[derive(Component, Default)]
pub struct Inventory{
    pub resource_inventory: HashMap<ResourceType, u32>, //stackable
    //weapon inventory or anything else would be a separate hashmap
}

fn read_resource_inventory(inventory: &Inventory) {
    if inventory.resource_inventory.is_empty() {
        info!("ResourceInventory is empty");
        return;
    }

    for (resource_type, count) in inventory.resource_inventory.iter() {
        info!("{}: {}", resource_type.name(), count);
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
    if let Some(cur_count) = inventory.resource_inventory.get_mut(&resource){
        *cur_count = count.saturating_sub(count);
    }
}