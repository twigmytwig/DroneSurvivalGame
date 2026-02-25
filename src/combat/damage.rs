//eventually will look for a damage event that will describe the entity
//and how much damage it took

use bevy::prelude::*;
use rand::{Rng, RngExt};
use crate::{
    player::Player,
    resources::DropTable,
    spawning::{DroneType, spawn_resources},
    state::GameState,
};

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
    player_query: Single<Entity, With<Player>>,
    mut death_messages: MessageReader<DeathEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    drone_query: Query<(&DroneType, &Transform)>,
    drop_table: Res<DropTable>,
    player: Single<Entity, With<Player>>,
) {
    let mut rng = rand::rng();

    for event in death_messages.read() {
        if event.entity.index() == player_query.index(){
            commands.entity(event.entity).try_despawn();
            //game over phase
            next_state.set(GameState::GameOver);
            continue; // Don't try to despawn again at end of loop
        }

        // Check if it's a drone and spawn resources
        if let Ok((drone_type, transform)) = drone_query.get(event.entity) {
            let pos = transform.translation.truncate();

            if let Some(drop_list) = drop_table.table.get(&drone_type.drone_type) {
                for drop in drop_list {
                    let count = rng.random_range(drop.min..=drop.max);
                    if count > 0 {
                        spawn_resources(&mut commands, drop.resource, pos, count, player.entity());
                    }
                }
            }
        }

        commands.entity(event.entity).try_despawn();
    }
}

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_message::<DamageEvent>()
            .add_message::<DeathEvent>()
            .add_systems(Update, apply_damage.run_if(in_state(GameState::Playing)))
            .add_systems(Update, apply_death.run_if(in_state(GameState::Playing)));
    }
}