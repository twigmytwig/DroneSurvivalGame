use bevy::prelude::*;

use crate::{inventory::Inventory, resources::ResourceDrop, state::GameState};
use super::inventory_component::add_resource;

const PICKUP_DISTANCE: f32 = 20.0;

//listen for a inventory pickup message
//take the resource entity -> get the resource type -> add it to inventory -> despawn

#[derive(Message)]
pub struct PickupEvent {
    pub collector: Entity,      // who picked it up (player or NPC)
    pub resource_entity: Entity, // the resource being collected
}

/// Detects when resources are close enough to a collector to be picked up
fn detect_resource_pickup(
    mut pickup_events: MessageWriter<PickupEvent>,
    collectors: Query<(Entity, &Transform), With<Inventory>>,
    resources: Query<(Entity, &Transform), With<ResourceDrop>>,
) {
    for (collector_entity, collector_transform) in &collectors {
        for (resource_entity, resource_transform) in &resources {
            let distance = resource_transform
                .translation
                .truncate()
                .distance(collector_transform.translation.truncate());

            if distance <= PICKUP_DISTANCE {
                pickup_events.write(PickupEvent {
                    collector: collector_entity,
                    resource_entity,
                });
            }
        }
    }
}

fn handle_pickup(
    mut commands: Commands,
    mut inventories: Query<&mut Inventory>,
    mut pickup_messages: MessageReader<PickupEvent>, 
    resources: Query<&ResourceDrop>,
) {
    for message in pickup_messages.read() {
        if let Ok(mut inventory) = inventories.get_mut(message.collector)
            && let Ok(resource) = resources.get(message.resource_entity)
        {
            add_resource(&mut inventory, resource.resource_type, 1);
            commands.entity(message.resource_entity).try_despawn();
            //spawn it into hotbar!
        }
    }
}

pub struct PickupPlugin;

impl Plugin for PickupPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<PickupEvent>()
            .add_systems(Update, (detect_resource_pickup, handle_pickup).chain().run_if(in_state(GameState::Playing)));
    }
}