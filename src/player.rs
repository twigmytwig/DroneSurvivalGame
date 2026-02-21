use bevy::prelude::*;
use crate::state::GameState;


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

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Player is spawned by level.rs from RON data (player_start)
        app.add_systems(Update, move_player.run_if(
            in_state(GameState::Playing))
        );
    }
}