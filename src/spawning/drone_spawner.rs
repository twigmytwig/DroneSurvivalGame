use bevy::prelude::*;
use crate::{
    ascii_sprite::AsciiSprite,
    combat::{Health, HealthBar},
    enemy::Enemy,
    npc_behaviors::{CollideTarget, ExplodeOnContact, MaintainRangeFromTarget, ShootAtTarget},
    physics::{CircleHitBox, DesiredDirection, DirectMovement, Velocity},
    spawning::{DroneType, MovementConfig}
};
use super::{DroneConfig, BehaviorConfig};

//turn config into entity
pub fn spawn_drone(
    commands: &mut Commands,
    config: &DroneConfig,
    pos: Vec2,
    target: Entity
) -> Entity {
    //make entity of drone 
    //Loop through behavior configs to attach additional components
    //spawn
    let mut entity = commands.spawn((
        Transform::from_translation(pos.extend(0.0)), //Transform expects Vec3, we have Vec2
        AsciiSprite{
            glyph: config.glyph.to_string(), 
            color: config.color, 
            font_size: config.font_size,
            bg_color: None //todo: implement this if i want to later
        },
        Enemy, //TODO: hard codded enemy probalby not good
        CircleHitBox { radius: config.hitbox_radius}, //TODO: HARDCODED BAD BAD WHAT IF OTHER TYPE OF HITBOX
        Velocity{speed: config.speed, direction: Vec2::ZERO},
        DesiredDirection::default(),
        Health::new(config.health),
        HealthBar {max_width: config.health_bar_width, offset: config.health_bar_offset},
        DroneType{drone_type: config.drone_type}
    ));

    match &config.movement{
        MovementConfig::Direct => {
            entity.insert(DirectMovement);
        },
        _ => {
            entity.insert(DirectMovement); //todo: figure out something else
        }
    }

    //Loop through behavior configs to attach additional components
    for behavior in &config.behaviors {
        match behavior {
            BehaviorConfig::CollideTarget => {
                entity.insert(CollideTarget { target });
            }
            BehaviorConfig::MaintainRange { range } => {
                entity.insert(MaintainRangeFromTarget { target, range: *range });
            }
            BehaviorConfig::ShootAtTarget { cooldown_secs, projectile } => {
                entity.insert(ShootAtTarget {
                    target,
                    cooldown: Timer::from_seconds(*cooldown_secs, TimerMode::Repeating),
                    config: projectile.clone(),
                });
            }
            BehaviorConfig::ExplodeOnContact { damage: _ } => {
                entity.insert(ExplodeOnContact);
                //TODO: store damage amount if needed per-drone
            }
        }
    }

    entity.id()
}