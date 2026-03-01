use bevy::prelude::*;

use crate::{
    combat::WeaponType,
    inventory::{Inventory, has_resources, remove_resource, add_weapon},
    resources::ResourceType,
};

//a recipe is a list of resources and each resource has a required amount.
//In the future we may want to look at recipes including more than just resources?

pub enum CraftableItem {
    Weapon(WeaponType),
    Beacon,
}

pub struct Recipe {
    pub name: &'static str,
    pub ingredients: &'static [(ResourceType, u32)],
    pub output: CraftableItem,
}

//Fun fact, these get baked into the binary at compile time because of &'static
const SHOTGUN_RECIPE: Recipe = Recipe {                
    name: "Shotgun",
    ingredients: &[(ResourceType::DroneWeaponParts, 1), (ResourceType::ScrapMetal, 2)],                              
    output: CraftableItem::Weapon(WeaponType::Shotgun),
};

const BEACON_RECIPE: Recipe = Recipe {
    name: "Extraction Beacon",
    ingredients: &[(ResourceType::Circuitry, 5), (ResourceType::DroneWeaponParts, 3)],
    output: CraftableItem::Beacon,
};

pub const ALL_RECIPES: &[Recipe] = &[SHOTGUN_RECIPE, BEACON_RECIPE];

/// Result of attempting to craft
pub enum CraftResult {
    Success,
    NotEnoughResources,
    AlreadyOwned, // for weapons that can't stack
}

/// Attempt to craft a recipe. Returns whether it succeeded.
pub fn try_craft(inventory: &mut Inventory, recipe: &Recipe) -> CraftResult {
    // Check if we have the resources
    if !has_resources(inventory, recipe.ingredients) {
        return CraftResult::NotEnoughResources;
    }

    // Deduct resources
    for (resource, count) in recipe.ingredients {
        remove_resource(inventory, *resource, *count);
    }

    // Add the crafted item
    match &recipe.output {
        CraftableItem::Weapon(weapon_type) => {
            if add_weapon(inventory, *weapon_type).is_none() {
                // Already owned - but we already deducted resources, this shouldn't happen
                // In a real game you'd check this before deducting
                return CraftResult::AlreadyOwned;
            }
        }
        CraftableItem::Beacon => {
            // TODO: Handle beacon crafting (trigger win condition or event)
            info!("Beacon crafted! Handle win condition here.");
        }
    }

    CraftResult::Success
}