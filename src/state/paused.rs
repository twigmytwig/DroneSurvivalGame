use bevy::prelude::*;

use crate::state::GameState;

#[derive(Component)]
pub struct PauseMenu;

#[derive(Component)]
pub enum PauseButton {
    Resume,
    Settings,
    Quit,
}

pub fn handle_button_clicks(
    query: Query<(&Interaction, &PauseButton), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>, //bevy defined resourece
    mut exit: MessageWriter<AppExit>,
){
    for (interaction, button) in &query{
        if *interaction == Interaction::Pressed{
            //this button was pressed
            match button{
                PauseButton::Resume => next_state.set(GameState::Playing),
                PauseButton::Settings => info!("TODO: implement settings screen"),
                PauseButton::Quit => {exit.write(AppExit::Success);},
                _ => info!("Nuttin"),
            }
        }
    }
}

pub fn spawn_pause_menu(mut commands: Commands){
    commands.spawn((
        PauseMenu,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
    )).with_children(|parent| {
        parent.spawn
        (
            (
            Text::new("Paused \nSettings stuff to come soon!"),
            TextFont {
                font_size: 36.0,
                ..default()
            },
            TextColor(Color::WHITE),
            TextLayout::new_with_justify(Justify::Center),
            ),

        );
        //resume button
        parent.spawn((
            Button,
            PauseButton::Resume,
            Node{
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.3,0.3,0.3)),
        )).with_children(|btn|{
            btn.spawn((
                Text::new("Resume"),
                TextFont{ font_size: 24.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });

        //settings button
        parent.spawn((
            Button,
            PauseButton::Settings,
            Node{
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.3,0.3,0.3)),
        )).with_children(|btn|{
            btn.spawn((
                Text::new("Settings"),
                TextFont{ font_size: 24.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });

        //quit button
        parent.spawn((
            Button,
            PauseButton::Quit,
            Node{
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.3,0.3,0.3)),
        )).with_children(|btn|{
            btn.spawn((
                Text::new("Quit"),
                TextFont{ font_size: 24.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });
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