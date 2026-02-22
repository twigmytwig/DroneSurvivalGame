use bevy::prelude::*;
use crate::physics::{CircleHitBox, circles_overlap};
use crate::player::Player;
use super::projectile::{Projectile, PlayerOwned, EnemyOwned};

// Player bullets hit enemies
// TODO: enable when Enemy component exists
// fn player_projectile_hits_enemy(
//     mut commands: Commands,
//     projectiles: Query<(Entity, &Transform, &CircleHitBox), (With<Projectile>, With<PlayerOwned>)>,
//     enemies: Query<(Entity, &Transform, &CircleHitBox), With<Enemy>>,
// ) {
//     for (proj_entity, proj_transform, proj_hitbox) in &projectiles {
//         for (enemy_entity, enemy_transform, enemy_hitbox) in &enemies {
//             if circles_overlap(
//                 proj_transform.translation.truncate(),
//                 proj_hitbox.radius,
//                 enemy_transform.translation.truncate(),
//                 enemy_hitbox.radius,
//             ) {
//                 commands.entity(proj_entity).despawn();
//                 info!("Player projectile hit enemy!");
//             }
//         }
//     }
// }

// Enemy bullets hit player
fn enemy_projectile_hits_player(
    mut commands: Commands,
    projectiles: Query<(Entity, &Transform, &CircleHitBox), (With<Projectile>, With<EnemyOwned>)>,
    player: Query<(Entity, &Transform, &CircleHitBox), With<Player>>,
) {
    let Ok((_, player_transform, player_hitbox)) = player.single() else {
        return;
    };

    for (proj_entity, proj_transform, proj_hitbox) in &projectiles {
        if circles_overlap(
            proj_transform.translation.truncate(),
            proj_hitbox.radius,
            player_transform.translation.truncate(),
            player_hitbox.radius,
        ) {
            commands.entity(proj_entity).despawn();
            info!("Enemy projectile hit player!"); // TODO HANDLE DAMAGE
        }
    }
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, enemy_projectile_hits_player);
    }
}