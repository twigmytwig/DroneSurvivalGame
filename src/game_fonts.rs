use bevy::prelude::*;

#[derive(Resource)]
pub struct GameFonts{
    pub mono: Handle<Font>,
}

pub fn load_fonts(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){
    commands.insert_resource(GameFonts{
        mono: asset_server.load("fonts/IBMPlexMono-Regular.ttf")
    });
}