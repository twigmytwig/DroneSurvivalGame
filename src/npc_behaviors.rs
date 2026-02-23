mod collide_target;

pub use collide_target::*;
use bevy::prelude::*;

pub struct NpcBehaviorPlugins;

impl Plugin for NpcBehaviorPlugins{
    fn build(&self, app: &mut App){
        app
        .add_plugins(collide_target::CollideTargetPlugin);
    }
}

