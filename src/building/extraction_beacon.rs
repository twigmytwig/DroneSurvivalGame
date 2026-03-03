use bevy::prelude::*;

use crate::state::GameState;

#[derive(Component)]
pub struct ExtractionBeacon(pub Timer);

impl ExtractionBeacon {
    pub fn new(secs: f32) -> Self {
        Self(Timer::from_seconds(secs, TimerMode::Once))
    }
}

//when the beacon is 'full charge' the player wins
fn tick_beacon_charge(
    mut beacon_query: Query<&mut ExtractionBeacon>, //can only place one?
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
){
    if let Ok(mut beacon) = beacon_query.single_mut(){
        beacon.0.tick(time.delta());
        if beacon.0.is_finished(){
            next_state.set(GameState::Victory);
        }
    } 
}