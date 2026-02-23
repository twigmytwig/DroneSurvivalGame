mod collide_target;
mod explode_on_contact;
mod shoot_at_target;
mod maintain_range;

pub use maintain_range::*;
pub use collide_target::*;
pub use explode_on_contact::*;
pub use shoot_at_target::*;
use bevy::prelude::*;

pub struct NpcBehaviorPlugins;

impl Plugin for NpcBehaviorPlugins{
    fn build(&self, app: &mut App){
        app
        .add_plugins(collide_target::CollideTargetPlugin)
        .add_plugins(shoot_at_target::ShootAtTargetPlugin)
        .add_plugins(maintain_range::MaintainRangePlugin);
    }
}

