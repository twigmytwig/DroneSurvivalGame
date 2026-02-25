use bevy::prelude::*;

pub fn play_sfx(
    commands: &mut Commands,
    asset_server: &AssetServer,
    name: &str,
    file_type: &str,
){
    let handle = asset_server
        .load(format!("sounds/sfx/{}.{}",name,file_type));
    commands.spawn((
        AudioPlayer::new(handle),
        PlaybackSettings::DESPAWN,
    ));
}