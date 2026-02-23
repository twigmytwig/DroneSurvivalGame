mod direct;
mod zig_zag;

pub use direct::*;
pub use zig_zag::*;

use bevy::prelude::*;

pub struct MovementStylesPlugin;

impl Plugin for MovementStylesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(direct::DirectMovementPlugin);
        // .add_plugins(zig_zag::ZigZagPlugin)  // TODO: when implemented
    }
}
