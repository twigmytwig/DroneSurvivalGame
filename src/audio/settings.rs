use bevy::prelude::*;

#[derive(Resource)]
pub struct AudioSettings{
    pub master: f32,
    pub music: f32,
    pub sfx: f32,
}