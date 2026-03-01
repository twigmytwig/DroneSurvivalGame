use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::state::GameState;
use crate::camera::GameCamera;
use crate::combat::{spawn_player_projectile, FirePattern, Weapon};


#[derive(Component)]
pub struct Player;

fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    player_query: Single<&mut Transform, With<Player>>,
)
{
    let mut player_transform = player_query.into_inner();

    let mut direction = Vec2::ZERO;
    if input.pressed(KeyCode::KeyA){
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD){
        direction.x += 1.0;
    }
    if input.pressed(KeyCode::KeyW){
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS){
        direction.y -= 1.0;
    }

    if direction != Vec2::ZERO{
        let speed = 300.0;
        let delta = direction.normalize() * speed * time.delta_secs();
        //Desired position because in the future there will be collision
        let desired_pos = Vec2::new(
            player_transform.translation.x + delta.x,
            player_transform.translation.y + delta.y,
        );

        player_transform.translation.x = desired_pos.x;
        player_transform.translation.y = desired_pos.y;
    }
}

fn player_shoot(
    mut commands: Commands,
    time: Res<Time>,
    input: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<GameCamera>>,
    player: Single<(&Transform, &mut Weapon), With<Player>>,
){
    let (transform, mut weapon) = player.into_inner();

    // Always tick cooldown
    weapon.fire_cooldown.tick(time.delta());

    // Fire if holding mouse AND cooldown ready
    if input.pressed(MouseButton::Left) && weapon.fire_cooldown.just_finished() {
        let (cam, cam_transform) = camera.into_inner();

        /*This was hard for me to wrap my little pea brain around so i will explain
        * window_cursor position gets where on the screen we clicked, useless on its own.
        * viewport_to_world translates where we clicked on monitor to that in the world.
        * It does that by taking the world point that the camera is looking at, and doing
        * math to what the screen click would equal to to world location. (returns Ray3d)
        * We then truncate that ray to get the vec 2.
        * Lastly, we do math to determine a direction in which we point at
        */
        if let Some(cursor_world) = window.cursor_position()
            .and_then(|cursor| cam.viewport_to_world(cam_transform, cursor).ok())
            .map(|ray| ray.origin.truncate())
        {
            let direction = (cursor_world - transform.translation.truncate()).normalize();

            match weapon.fire_pattern {
                FirePattern::Single => {
                    spawn_player_projectile(
                        &mut commands,
                        transform.translation.truncate(),
                        direction,
                        &weapon.config,
                    );
                }
                FirePattern::Spread { count, angle_degrees } => {
                    let total_rad = angle_degrees.to_radians();
                    let step = total_rad / (count - 1).max(1) as f32;
                    let start = -total_rad / 2.0;

                    for i in 0..count {
                        let angle = start + step * i as f32;
                        let rotated = Vec2::new(
                            direction.x * angle.cos() - direction.y * angle.sin(),
                            direction.x * angle.sin() + direction.y * angle.cos(),
                        );
                        spawn_player_projectile(
                            &mut commands,
                            transform.translation.truncate(),
                            rotated,
                            &weapon.config,
                        );
                    }
                }
            }
        }
    }
}

fn player_interact(
    input: Res<ButtonInput<KeyCode>>,
){
    if input.just_released(KeyCode::KeyE){
        info!("player_interact no implemented");
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Player is spawned by level.rs from RON data (player_start)
        app.add_systems(Update, (
            move_player.run_if(
            in_state(GameState::Playing)
            ),
            player_shoot.run_if(
                in_state(GameState::Playing)
            ),
            player_interact.run_if(
                in_state(GameState::Playing)
            ),
        )
        );
    }
}