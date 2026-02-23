use bevy::prelude::*;
mod ascii_sprite;
mod camera;
mod combat;
mod game_fonts;
mod helpers;
mod physics;
mod player;
mod state;
mod npc_behaviors;
mod enemy;

use camera::CameraPlugin;
use combat::CombatPlugin;
use physics::PhysicsPlugin;
use player::PlayerPlugin;
use ascii_sprite::{render_ascii_sprites, test};

use crate::npc_behaviors::NpcBehaviorPlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(state::StatePlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(CombatPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(NpcBehaviorPlugins)
        .add_systems(Startup, test)
        .add_systems(Update, render_ascii_sprites)
        .run();
}

