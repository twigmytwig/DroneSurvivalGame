//eventually will look for a damage event that will describe the entity
//and how much damage it took

use bevy::prelude::*;

#[derive(Component)]
pub struct ProjectileDamage(pub u32);

#[derive(Message)]
pub struct DamageEvent {
    pub target: Entity,
    pub amount: u32,
}

fn apply_damage(
    mut messages: MessageReader<DamageEvent>,
) {
    for event in messages.read() {
        info!("Damage event: {:?} took {} damage", event.target, event.amount);
        // TODO: actually apply damage when Health component exists
    }
}

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<DamageEvent>()
           .add_systems(Update, apply_damage);
    }
}