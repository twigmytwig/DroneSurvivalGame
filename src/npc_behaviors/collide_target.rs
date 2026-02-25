use bevy::prelude::*;
use crate::{physics::DesiredDirection, state::GameState};

#[derive(Component)]
  pub struct CollideTarget {
      pub target: Entity,
  }

//Following a target to the point of collision. 
//Updating the desired direction to the new transform of the target
fn collide_target_system(
    targets: Query<&Transform>,
    mut chasers: Query<(&CollideTarget, &Transform, &mut DesiredDirection)>,
) {
    for (collide, transform, mut desired) in &mut chasers {
        // Get the target's transform using the Entity ID
        if let Ok(target_transform) = targets.get(collide.target) {
            desired.0 = (target_transform.translation - transform.translation)
                .truncate()
                .normalize_or_zero();
        }
    }
}

pub struct CollideTargetPlugin;

impl Plugin for CollideTargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collide_target_system.run_if(in_state(GameState::Playing)));
    }
}