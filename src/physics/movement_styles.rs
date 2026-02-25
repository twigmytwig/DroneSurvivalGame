mod direct;
mod zig_zag;
mod magnetizable;

pub use direct::*;
pub use zig_zag::*;
pub use magnetizable::*;

use bevy::prelude::*;

pub struct MovementStylesPlugin;

impl Plugin for MovementStylesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(direct::DirectMovementPlugin);
        app.add_plugins(magnetizable::MagnetismPlugin);
        // .add_plugins(zig_zag::ZigZagPlugin)  // TODO: when implemented
    }
}
