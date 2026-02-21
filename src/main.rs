use bevy::prelude::*;
mod ascii_sprite;
mod game_fonts;
mod state;
mod player;
mod camera;
mod helpers;
use crate::camera::CameraPlugin;
use crate::player::PlayerPlugin;
use ascii_sprite::{render_ascii_sprites, test};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(state::StatePlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .add_systems(Startup, test)
        .add_systems(Update, render_ascii_sprites)
        .run();
}

