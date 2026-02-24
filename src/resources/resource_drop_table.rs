use bevy::prelude::*;
use std::collections::HashMap;
use crate::spawning::DroneKind;
use super::ResourceType;

//this is where we determine who can drop which resources
//and use random to determine how much to drop
//We will pair the DroneKind enum to the resourceTypes enum
pub struct ResourceRange{
    pub resource: ResourceType,
    pub min: u32,
    pub max: u32,
}

#[derive(Resource)]
pub struct DropTable {
    pub table: HashMap<DroneKind, Vec<ResourceRange>>,
}

impl Default for DropTable {
    fn default() -> Self {
        let mut table = HashMap::new();
        //populating table
        table.insert(DroneKind::Chaser, vec![
            ResourceRange{resource: ResourceType::ScrapMetal, min: 1, max: 2},
        ]);

        table.insert(DroneKind::Shooter, vec![
            ResourceRange { resource: ResourceType::ScrapMetal, min: 1, max: 2 },
            ResourceRange { resource: ResourceType::DroneWeaponParts, min: 0, max: 1 },
        ]);
        Self { table }
    }
}