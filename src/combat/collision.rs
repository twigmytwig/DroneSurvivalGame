use bevy::prelude::*;
use crate::combat::Dead;
use crate::enemy::Enemy;
use crate::npc_behaviors::ExplodeOnContact;
use crate::physics::{CircleHitBox, circles_overlap};
use crate::player::Player;
use super::projectile::{Projectile, PlayerOwned, EnemyOwned};
use super::damage::{ProjectileDamage, DamageEvent};

// Player bullets hit enemies
fn player_projectile_hits_enemy(
    mut commands: Commands,
    mut damage_messages: MessageWriter<DamageEvent>,
    projectiles: Query<(Entity, &Transform, &CircleHitBox, &ProjectileDamage), (With<Projectile>, With<PlayerOwned>)>,
    enemies: Query<(Entity, &Transform, &CircleHitBox), With<Enemy>>,
) {
    for (proj_entity, proj_transform, proj_hitbox, damage) in &projectiles {
        for (enemy_entity, enemy_transform, enemy_hitbox) in &enemies {
            if circles_overlap(
                proj_transform.translation.truncate(),
                proj_hitbox.radius,
                enemy_transform.translation.truncate(),
                enemy_hitbox.radius,
            ) {
                commands.entity(proj_entity).despawn();
                info!("Player projectile hit enemy!");
                damage_messages.write(DamageEvent {
                    target: enemy_entity,
                    amount: damage.0,
                });
                break; // Projectile can only hit one enemy
            }
        }
    }
}

// Enemy bullets hit player
fn enemy_projectile_hits_player(
    mut commands: Commands,
    mut damage_messages: MessageWriter<DamageEvent>,
    projectiles: Query<(Entity, &Transform, &CircleHitBox, &ProjectileDamage), (With<Projectile>, With<EnemyOwned>)>,
    player: Query<(Entity, &Transform, &CircleHitBox), With<Player>>,
) {
    let Ok((player_entity, player_transform, player_hitbox)) = player.single() else {
        return;
    };

    for (proj_entity, proj_transform, proj_hitbox, proj_damage) in &projectiles {
        if circles_overlap(
            proj_transform.translation.truncate(),
            proj_hitbox.radius,
            player_transform.translation.truncate(),
            player_hitbox.radius,
        ) {
            commands.entity(proj_entity).despawn();
            info!("Enemy projectile hit player!");
            damage_messages.write(DamageEvent {
                target: player_entity,
                amount: proj_damage.0,
            });
        }
    }
}

fn enemy_collides_with_player(
    mut commands: Commands,
    mut damage_messages: MessageWriter<DamageEvent>,
    enemies: Query<(Entity, &Transform, &CircleHitBox, Option<&ExplodeOnContact>), (With<Enemy>, Without<Dead>)>,
    player: Query<(Entity, &Transform, &CircleHitBox), With<Player>>,
) {
    let Ok((player_entity, player_transform, player_hitbox)) = player.single() else {
        return;
    };

    for (enemy_entity, enemy_transform, enemy_hitbox, explode) in &enemies {
        if circles_overlap(
            enemy_transform.translation.truncate(),
            enemy_hitbox.radius,
            player_transform.translation.truncate(),
            player_hitbox.radius,
        ) {
            info!("Enemy collided with player!");
            if explode.is_some(){
                info!("Exploding enemy collided with player!");
                commands.entity(enemy_entity).insert(Dead);// this is to prevent multiple collison events, breaking the game
                damage_messages.write(DamageEvent {
                    target: player_entity,
                    amount: 5,
                });
                // Enemy self-destructs (damage itself for its full health)
                damage_messages.write(DamageEvent {
                    target: enemy_entity,
                    amount: 9999,  // or query enemy's health.max
                });
            }
            //collision happend but wasnt an exploder
            
        }
    }
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, enemy_projectile_hits_player)
            .add_systems(Update, player_projectile_hits_enemy)
            .add_systems(Update, enemy_collides_with_player);
    }
}