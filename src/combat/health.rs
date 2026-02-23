//todo health! this will end up being like health bars that appear above the
//entities and what not.
use bevy::prelude::*;

#[derive(Component)]
pub struct Dead;

#[derive(Component)]
pub struct HealthBar {
    pub max_width: f32,
    pub offset: f32,
}

#[derive(Component)]
pub struct HealthBarSpawned;  // Marker: bar already created

#[derive(Component)]
pub struct HealthBarFill {
    pub max_width: f32,
}

#[derive(Component)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}

// Find everyone with Health + HealthBar but no HealthBarSpawned, and attach visual
fn attach_health_bars(
    mut commands: Commands,
    query: Query<(Entity, &HealthBar), (With<Health>, Without<HealthBarSpawned>)>,
) {
    for (entity, bar) in &query {
        commands.entity(entity)
            .insert(HealthBarSpawned)
            .with_children(|parent| {
                // Background (gray)
                parent.spawn((
                    Sprite {
                        color: Color::srgb(0.3, 0.3, 0.3),
                        custom_size: Some(Vec2::new(bar.max_width, 6.0)),
                        ..default()
                    },
                    Transform::from_xyz(0.0, bar.offset, 0.0),
                ));

                // Foreground (green fill)
                parent.spawn((
                    Sprite {
                        color: Color::srgb(0.0, 1.0, 0.0),
                        custom_size: Some(Vec2::new(bar.max_width, 6.0)),
                        ..default()
                    },
                    Transform::from_xyz(0.0, bar.offset, 0.1),
                    HealthBarFill { max_width: bar.max_width },
                ));
            });
    }
}

// Update health bar width based on current health
fn update_health_bars(
    health_query: Query<&Health>,
    mut bar_query: Query<(&ChildOf, &HealthBarFill, &mut Sprite)>,
) {
    for (child, fill, mut sprite) in &mut bar_query {
        if let Ok(health) = health_query.get(child.parent()) {
            let percent = health.current as f32 / health.max as f32;
            sprite.custom_size = Some(Vec2::new(fill.max_width * percent, 6.0));
        }
    }
}

impl Health {
    pub fn new(max: u32) -> Self {
        Self { current: max, max }
    }
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (attach_health_bars, update_health_bars));
    }
}