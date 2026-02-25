use bevy::prelude::*;

/// Marker component to identify music entities for stopping later
#[derive(Component)]
pub struct MusicTrack;

/// Plays looping music, returns the entity so it can be stopped later
pub fn play_music(
    commands: &mut Commands,
    asset_server: &AssetServer,
    path: &str,
) -> Entity {
    commands.spawn((
        AudioPlayer::new(asset_server.load(path.to_string())),
        PlaybackSettings::LOOP,
        MusicTrack,
    )).id()
}

/// Stops all music with the MusicTrack marker
pub fn stop_music(
    mut commands: Commands,
    music_query: Query<Entity, With<MusicTrack>>,
) {
    for entity in &music_query {
        commands.entity(entity).despawn();
    }
}
