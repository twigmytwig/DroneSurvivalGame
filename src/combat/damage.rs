//eventually will look for a damage event that will describe the entity
//and how much damage it took

use bevy::prelude::*;
use super::health::Health;

#[derive(Message)]
pub struct DeathEvent{
    pub entity: Entity,
}

#[derive(Component)]
pub struct ProjectileDamage(pub u32);

#[derive(Message)]
pub struct DamageEvent {
    pub target: Entity,
    pub amount: u32,
}

fn apply_damage(
    mut messages: MessageReader<DamageEvent>,
    mut death_messages: MessageWriter<DeathEvent>,
    mut health_query: Query<&mut Health>,
) {
    for event in messages.read() {
        if let Ok(mut health) = health_query.get_mut(event.target){
            //saturating sub clamps to 0
            health.current = health.current.saturating_sub(event.amount);
            info!("Damage event: {:?} took {} damage", event.target, event.amount);
            if health.current == 0{
                death_messages.write(DeathEvent { entity: event.target });
            }
        }
    }
}

fn apply_death(
    mut commands: Commands,
    mut death_messages: MessageReader<DeathEvent>,
) {
    for event in death_messages.read() {
        commands.entity(event.entity).despawn();
    }
}

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_message::<DamageEvent>()
            .add_message::<DeathEvent>()
            .add_systems(Update, apply_damage)
            .add_systems(Update, apply_death);
    }
}