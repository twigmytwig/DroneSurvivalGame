use bevy::prelude::*;

#[derive(Component)]
pub struct CircleHitBox{
    pub radius: f32,
}

pub fn circles_overlap(pos_a: Vec2, radius_a: f32, pos_b: Vec2, radius_b: f32) -> bool {
    pos_a.distance(pos_b) < radius_a + radius_b
}

pub fn debug_draw_circle_hitboxes(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &CircleHitBox)>,
) {
    for (transform, hitbox) in &query {
        gizmos.circle_2d(
            transform.translation.truncate(),  // center position
            hitbox.radius,                      // radius
            Color::srgb(0.0, 1.0, 0.0),         // green
        );
    }
}

pub struct HitboxPlugin;

impl Plugin for HitboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debug_draw_circle_hitboxes);
    }
}