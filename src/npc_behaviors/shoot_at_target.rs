//firing a projectile at a target

use bevy::prelude::*;
use crate::{combat::{spawn_enemy_projectile, ProjectileConfig}, state::GameState};

#[derive(Component)]
pub struct ShootAtTarget {
    pub target: Entity,
    pub cooldown: Timer,
    pub config: ProjectileConfig,
}

fn shoot_at_target_system(
    mut commands: Commands,
    time: Res<Time>,
    targets: Query<&Transform>,
    mut shooters: Query<(&mut ShootAtTarget, &Transform)>,
) {
    for (mut shoot, transform) in &mut shooters {
        shoot.cooldown.tick(time.delta());

        if shoot.cooldown.just_finished() {
            // Get target position
            if let Ok(target_transform) = targets.get(shoot.target) {
                let direction = (target_transform.translation - transform.translation)
                    .truncate()
                    .normalize_or_zero();

                spawn_enemy_projectile(
                    &mut commands,
                    transform.translation.truncate(),
                    direction,
                    &shoot.config,
                );
            }
        }
    }
}

pub struct ShootAtTargetPlugin;

impl Plugin for ShootAtTargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, shoot_at_target_system.run_if(in_state(GameState::Playing)));
    }
}