use bevy::prelude::*;
use crate::game_fonts::GameFonts;

#[derive(Component)]
pub struct AsciiSprite {
    pub glyph: String,
    pub color: Color,
    pub font_size: f32,
    pub bg_color: Option<Color>, //potential background color for the future?
}

//Added<AsciiSprite> filter - Only runs when new AsciiSprite components are added
//not every frame which is fuckin cool
pub fn render_ascii_sprites(
    mut commands: Commands,
    fonts: Res<GameFonts>,
    query: Query<(Entity, &AsciiSprite), Added<AsciiSprite>>,
) {
    for (entity, sprite) in &query {
        commands.entity(entity).insert((
            Text2d::new(sprite.glyph.to_string()),
            TextFont { 
                font: fonts.mono.clone(),
                font_size: sprite.font_size,
                ..default()
            },
            TextColor(sprite.color),
        ));
    }
}

pub fn test(
    mut commands: Commands
){
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),                                                                  
        AsciiSprite {
            glyph: "<(=)>".to_string(),  // 5 characters wide
            color: Color::WHITE,
            font_size: 24.0,
            bg_color: None
        },
    ));
}