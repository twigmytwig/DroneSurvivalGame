use bevy::prelude::*;

#[derive(Component)]
pub struct PauseMenu;

pub fn spawn_pause_menu(mut commands: Commands){
    commands.spawn((
        PauseMenu,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
    )).with_children(|parent| {
        parent.spawn((
            Text::new("Paused \nSettings stuff to come soon!"),
            TextFont {
                font_size: 36.0,
                ..default()
            },
            TextColor(Color::WHITE),
            TextLayout::new_with_justify(Justify::Center),
        ));
    });
}

pub fn despawn_pause_menu(
    mut commands: Commands,
    query: Query<Entity, With<PauseMenu>>,
){
    for entity in query.iter(){
        commands.entity(entity).despawn();
    }
}

pub fn handle_quit(){

}

pub fn handle_play(){

}

pub fn handle_settings(){

}