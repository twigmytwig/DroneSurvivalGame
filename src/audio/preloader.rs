use bevy::prelude::*;

#[derive(Resource)]
pub struct GameAudios {
    // SFX
    pub player_shoot: Handle<AudioSource>,
    pub character_hit: Handle<AudioSource>,
    pub player_hit_explosion: Handle<AudioSource>,

    // Music
    pub game_over_music: Handle<AudioSource>,
    pub victory_music: Handle<AudioSource>,
}

pub fn load_audios(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAudios {
        // SFX
        player_shoot: asset_server.load("sounds/sfx/player_shoot.mp3"),
        character_hit: asset_server.load("sounds/sfx/character_hit.mp3"),
        player_hit_explosion: asset_server.load("sounds/sfx/player_hit_explosion.mp3"),

        // Music
        game_over_music: asset_server.load("sounds/music/game_over_track.mp3"),
        victory_music: asset_server.load("sounds/music/action_man.mp3"),
    });
}
