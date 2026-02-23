// maintain a certain range to a target
use bevy::prelude::*;
use crate::physics::DesiredDirection;

#[derive(Component)]
pub struct MaintainRangeFromTarget {
    pub target: Entity,
    pub range: f32,
}

fn maintain_range_system(
    targets: Query<&Transform>,
    mut chasers: Query<(&MaintainRangeFromTarget, &Transform, &mut DesiredDirection)>,
) {
    const TOLERANCE: f32 = 10.0; // Dead zone to prevent stuttering

    for (maintain, transform, mut desired) in &mut chasers {
        if let Ok(target_transform) = targets.get(maintain.target) {
            let my_pos = transform.translation.truncate();
            let target_pos = target_transform.translation.truncate();
            let distance = my_pos.distance(target_pos);

            if distance > maintain.range + TOLERANCE {
                // Too far - move toward target
                desired.0 = (target_pos - my_pos).normalize_or_zero();
            } else if distance < maintain.range - TOLERANCE {
                // Too close - move away from target
                desired.0 = (my_pos - target_pos).normalize_or_zero();
            } else {
                // Within tolerance - stop
                desired.0 = Vec2::ZERO;
            }
        }
    }
}

pub struct MaintainRangePlugin;

impl Plugin for MaintainRangePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, maintain_range_system);
    }
}