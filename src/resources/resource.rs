use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    ScrapMetal,
    Circuitry,
    DroneWeaponParts,
}

impl ResourceType {
    pub fn name(&self) -> &'static str {
        match self {
            ResourceType::ScrapMetal => "Scrap Metal",
            ResourceType::Circuitry => "Circuitry",
            ResourceType::DroneWeaponParts => "Drone Weapon Parts",
        }
    }

    pub fn glyph(&self) -> &'static str {
        match self {
            ResourceType::ScrapMetal => "#",
            ResourceType::Circuitry => "&",
            ResourceType::DroneWeaponParts => "%",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            ResourceType::ScrapMetal => Color::srgb(0.5, 0.5, 0.5), // Gray
            ResourceType::Circuitry => Color::srgb(0.0, 1.0, 1.0),  // Cyan
            ResourceType::DroneWeaponParts => Color::srgb(1.0, 0.5, 0.0), // Orange
        }
    }
}

#[derive(Component)]
pub struct ResourceDrop {
    pub resource_type: ResourceType,
}

//how long a resource can be on the ground before despawn
#[derive(Component)]
pub struct ResourceLifeTimer(pub Timer);

const BLINK_THRESHOLD_SECS: f32 = 10.0;
const BLINK_INTERVAL_SECS: f32 = 0.2;

fn tick_resource_lifetimes(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ResourceLifeTimer, &mut Visibility)>,
) {
    for (entity, mut timer, mut visibility) in &mut query {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            commands.entity(entity).despawn();
            continue;
        }

        // Blink when under 10 seconds remaining
        let remaining = timer.0.duration().as_secs_f32() - timer.0.elapsed_secs();
        if remaining <= BLINK_THRESHOLD_SECS {
            // Toggle visibility based on time intervals
            let blink_on = (remaining / BLINK_INTERVAL_SECS) as i32 % 2 == 0;
            *visibility = if blink_on {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
    }
}

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick_resource_lifetimes);
    }
}