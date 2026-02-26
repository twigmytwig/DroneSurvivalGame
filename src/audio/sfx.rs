use bevy::prelude::*;
use bevy::audio::Volume;

use crate::audio::AudioSettings;

pub fn play_sfx(
    commands: &mut Commands,
    asset_server: &AssetServer,
    name: &str,
    file_type: &str,
    sound_settings: &AudioSettings,
){
    let handle = asset_server
        .load(format!("sounds/sfx/{}.{}",name,file_type));
    commands.spawn((
        AudioPlayer::new(handle),
        PlaybackSettings::DESPAWN.with_volume(Volume::Linear(sound_settings.sfx/100.0)),
    ));
}