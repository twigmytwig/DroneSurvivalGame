use bevy::prelude::*;

pub fn player_interact(
    input: Res<ButtonInput<KeyCode>>,
){
    if input.just_released(KeyCode::KeyE){
        info!("player_interact no implemented");
    }
}
