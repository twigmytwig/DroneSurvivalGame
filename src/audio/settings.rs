use bevy::prelude::*;

#[derive(Resource)]
pub struct AudioSettings {
    pub master: f32,
    pub music: f32,
    pub sfx: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        AudioSettings { master: 100.0, music: 100.0, sfx: 100.0 }
    }
}