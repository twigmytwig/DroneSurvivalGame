use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::camera::GameCamera;
use crate::player::Player;
use super::grid::{BuildGrid, world_to_grid, grid_to_world, GRID_CELL_SIZE};
use super::placeable::{PlaceableConfig, PlaceableType, spawn_structure};

const GRID_EXTENT: i32 = 5; // 11x11 grid (5 cells in each direction from center)
const LINE_COLOR: Color = Color::srgba(1.0, 1.0, 1.0, 0.15);
const LINE_THICKNESS: f32 = 1.0;

#[derive(Resource, Default)]
pub struct BuildMode {
    pub selected: Option<PlaceableType>,
}

#[derive(Component)]
pub struct GridOverlay;

#[derive(Component)]
pub struct GhostPreview;

/// B key toggles build mode on/off
pub fn toggle_build_mode(
    input: Res<ButtonInput<KeyCode>>,
    mut build_mode: ResMut<BuildMode>,
) {
    if input.just_pressed(KeyCode::KeyB) {
        if build_mode.selected.is_some() {
            build_mode.selected = None;
            info!("Build mode OFF");
        } else {
            // TODO: check player inventory for placeables
            build_mode.selected = Some(PlaceableType::ExtractionBeacon);
            info!("Build mode ON — Extraction Beacon selected");
        }
    }
}

/// Spawn grid lines + ghost preview when build mode turns on, despawn when it turns off
pub fn manage_grid_overlay(
    mut commands: Commands,
    build_mode: Res<BuildMode>,
    overlay_query: Query<Entity, With<GridOverlay>>,
    ghost_query: Query<Entity, With<GhostPreview>>,
    player_query: Query<&Transform, With<Player>>,
) {
    // Only react when BuildMode actually changes
    if !build_mode.is_changed() {
        return;
    }

    if let Some(ref selected) = build_mode.selected {
        // Don't spawn a second overlay if one exists
        if !overlay_query.is_empty() {
            return;
        }

        let config = PlaceableConfig::from_type(selected);
        let cell = GRID_CELL_SIZE as f32;
        let line_count = GRID_EXTENT * 2 + 2; // lines = cells + 1
        let total_size = cell * (GRID_EXTENT * 2 + 1) as f32;

        // Spawn parent entity at player position, snapped to grid
        let player_pos = player_query
            .single()
            .map(|t| t.translation.truncate())
            .unwrap_or(Vec2::ZERO);
        let snapped = grid_to_world(world_to_grid(player_pos));

        commands.spawn((
            Transform::from_translation(snapped.extend(0.0)),
            GridOverlay,
        )).with_children(|parent| {
            // Vertical lines
            for i in 0..line_count {
                let x = (i - GRID_EXTENT) as f32 * cell - cell / 2.0;
                parent.spawn((
                    Sprite {
                        color: LINE_COLOR,
                        custom_size: Some(Vec2::new(LINE_THICKNESS, total_size)),
                        ..default()
                    },
                    Transform::from_xyz(x, 0.0, -1.0),
                ));
            }
            // Horizontal lines
            for i in 0..line_count {
                let y = (i - GRID_EXTENT) as f32 * cell - cell / 2.0;
                parent.spawn((
                    Sprite {
                        color: LINE_COLOR,
                        custom_size: Some(Vec2::new(total_size, LINE_THICKNESS)),
                        ..default()
                    },
                    Transform::from_xyz(0.0, y, -1.0),
                ));
            }
        });

        // Spawn ghost preview
        commands.spawn((
            Transform::from_translation(snapped.extend(1.0)),
            crate::ascii_sprite::AsciiSprite {
                glyph: config.glyph.clone(),
                color: Color::srgba(0.0, 1.0, 0.0, 0.5),
                font_size: config.font_size,
                bg_color: None,
            },
            GhostPreview,
        ));
    } else {
        // Despawn overlay (children auto-despawn)
        for entity in &overlay_query {
            commands.entity(entity).despawn();
        }
        // Despawn ghost
        for entity in &ghost_query {
            commands.entity(entity).despawn();
        }
    }
}

/// Keep grid overlay centered on player, snapped to grid alignment
pub fn update_grid_overlay_position(
    build_mode: Res<BuildMode>,
    player_query: Query<&Transform, With<Player>>,
    mut overlay_query: Query<&mut Transform, (With<GridOverlay>, Without<Player>)>,
) {
    if build_mode.selected.is_none() {
        return;
    }

    let Ok(player_transform) = player_query.single() else { return };
    let Ok(mut overlay_transform) = overlay_query.single_mut() else { return };

    let snapped = grid_to_world(world_to_grid(player_transform.translation.truncate()));
    overlay_transform.translation.x = snapped.x;
    overlay_transform.translation.y = snapped.y;
}

/// Ghost preview follows cursor, snapped to grid. Green if free, red if occupied.
pub fn update_ghost_preview(
    build_mode: Res<BuildMode>,
    build_grid: Res<BuildGrid>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    mut ghost_query: Query<(&mut Transform, &mut TextColor), With<GhostPreview>>,
) {
    if build_mode.selected.is_none() {
        return;
    }

    let Ok(window) = window.single() else { return };
    let Ok((cam, cam_transform)) = camera.single() else { return };
    let Ok((mut ghost_transform, mut text_color)) = ghost_query.single_mut() else { return };

    let Some(cursor_world) = window.cursor_position()
        .and_then(|cursor| cam.viewport_to_world(cam_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    else { return };

    let grid_pos = world_to_grid(cursor_world);
    let snapped = grid_to_world(grid_pos);
    ghost_transform.translation.x = snapped.x;
    ghost_transform.translation.y = snapped.y;

    // Green if cell is free, red if occupied
    if build_grid.occupied_cells.contains_key(&grid_pos) {
        *text_color = TextColor(Color::srgba(1.0, 0.0, 0.0, 0.5));
    } else {
        *text_color = TextColor(Color::srgba(0.0, 1.0, 0.0, 0.5));
    }
}

/// Left-click in build mode: place a structure on the grid
pub fn place_structure(
    mut commands: Commands,
    mut build_mode: ResMut<BuildMode>,
    mut build_grid: ResMut<BuildGrid>,
    input: Res<ButtonInput<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
) {
    let Some(ref selected) = build_mode.selected else { return };
    if !input.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = window.single() else { return };
    let Ok((cam, cam_transform)) = camera.single() else { return };

    let Some(cursor_world) = window.cursor_position()
        .and_then(|cursor| cam.viewport_to_world(cam_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    else { return };

    let grid_pos = world_to_grid(cursor_world);

    // Can't place on an occupied cell
    if build_grid.occupied_cells.contains_key(&grid_pos) {
        info!("Cell {:?} is occupied!", grid_pos);
        return;
    }

    // Spawn the structure
    let config = PlaceableConfig::from_type(selected);
    let world_pos = grid_to_world(grid_pos);
    let entity = spawn_structure(&mut commands, world_pos, &config);

    // Register in grid
    build_grid.occupied_cells.insert(grid_pos, entity);
    info!("Placed {:?} at {:?}", config.name, grid_pos);

    // TODO: consume from player inventory, exit build mode if they run out
    // For now, exit build mode after placing
    build_mode.selected = None;
}
