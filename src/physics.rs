// physics.rs
  mod velocity;
  mod hitbox;

  pub use velocity::*;
  pub use hitbox::*;

  use bevy::prelude::*;

  pub struct PhysicsPlugin;

  impl Plugin for PhysicsPlugin {
      fn build(&self, app: &mut App) {
          app.add_plugins(velocity::VelocityPlugin);
      }
  }