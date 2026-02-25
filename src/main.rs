use bevy::prelude::*;
mod ascii_sprite;
mod camera;
mod combat;
mod enemy;
mod game_fonts;
mod helpers;
mod npc_behaviors;
mod physics;
mod player;
mod resources;
mod spawning;
mod state;
mod inventory;
mod audio;

use camera::CameraPlugin;
use combat::CombatPlugin;
use inventory::PickupPlugin;
use physics::PhysicsPlugin;
use player::PlayerPlugin;
use ascii_sprite::render_ascii_sprites;
use resources::{DropTable, ResourcePlugin};

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
        .add_plugins(ResourcePlugin)
        .add_plugins(PickupPlugin)
        .init_resource::<DropTable>()
        .add_systems(Update, render_ascii_sprites)
        .run();
}

