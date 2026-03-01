use bevy::prelude::*;
use crate::combat::Weapon;
use crate::inventory::{Inventory, weapon_at_slot};
use super::Player;

pub fn weapon_switch(
    input: Res<ButtonInput<KeyCode>>,
    mut player: Single<(&mut Weapon, &mut Inventory), With<Player>>,
){
    let slot = if input.just_pressed(KeyCode::Digit1) {
        Some(0)
    } else if input.just_pressed(KeyCode::Digit2) {
        Some(1)
    } else if input.just_pressed(KeyCode::Digit3) {
        Some(2)
    } else {
        None
    };

    if let Some(slot) = slot {
        let (ref mut weapon, ref mut inventory) = *player;
        if let Some(weapon_type) = weapon_at_slot(inventory, slot) {
            inventory.active_weapon_slot = slot;
            *weapon.as_mut() = Weapon::from_type(weapon_type);
        }
    }
}
