use bevy::prelude::*;
use bevy::ecs::hierarchy::ChildSpawnerCommands;

use crate::crafting::try_craft;
use crate::inventory::has_resources;
use crate::state::GameState;
use crate::inventory::Inventory;
use crate::player::Player;
use super::recipe::{Recipe, ALL_RECIPES};

// =============================================================================
// MARKER COMPONENTS
// =============================================================================

#[derive(Component)]
pub struct CraftingMenu;

/// Identifies which recipe a craft button is for (by index into ALL_RECIPES)
#[derive(Component)]
pub struct CraftButton(pub usize);

#[derive(Component)]
pub struct CloseButton;

/// Marks an ingredient text for dynamic updates (recipe_index, ingredient_index)
#[derive(Component)]
pub struct IngredientText(pub usize, pub usize);

/// Marks a craft button's background for dynamic color updates
#[derive(Component)]
pub struct CraftButtonBg(pub usize);

// =============================================================================
// TOGGLE CRAFTING (Tab key)
// =============================================================================

pub fn toggle_crafting(
    input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::Tab) {
        match current_state.get() {
            GameState::Playing => {
                next_state.set(GameState::Crafting);
            }
            GameState::Crafting => {
                next_state.set(GameState::Playing);
            }
            _ => {}
        }
    }
}

// =============================================================================
// SPAWN / DESPAWN
// =============================================================================

pub fn spawn_crafting_ui(
    mut commands: Commands,
    player_query: Query<&Inventory, With<Player>>,
) {
    let player_inventory = player_query.single().ok();

    commands.spawn((
        CraftingMenu,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
    )).with_children(|parent| {
        // Title
        parent.spawn((
            Text::new("CRAFTING"),
            TextFont { font_size: 48.0, ..default() },
            TextColor(Color::WHITE),
        ));

        // Recipe list container
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(15.0),
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
        )).with_children(|list| {
            // Spawn a row for each recipe
            for (index, recipe) in ALL_RECIPES.iter().enumerate() {
                spawn_recipe_row(list, index, recipe, player_inventory);
            }
        });

        // Close button
        parent.spawn((
            Button,
            CloseButton,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::top(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
        )).with_children(|btn| {
            btn.spawn((
                Text::new("Close [Tab]"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });
    });
}

/// Spawns a single recipe row: [Name] [Ingredients] [Craft Button]
fn spawn_recipe_row(
    parent: &mut ChildSpawnerCommands,
    index: usize,
    recipe: &Recipe,
    player_inventory: Option<&Inventory>,
) {
    let can_craft = check_can_craft(recipe, player_inventory);

    parent.spawn((
        Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(20.0),
            padding: UiRect::all(Val::Px(10.0)),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BorderColor::all(Color::srgb(0.4, 0.4, 0.4)),
    )).with_children(|row| {
        // Recipe name
        row.spawn((
            Text::new(recipe.name),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::WHITE),
            Node {
                width: Val::Px(180.0),
                ..default()
            },
        ));

        // Ingredients list
        row.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.0),
                width: Val::Px(200.0),
                ..default()
            },
        )).with_children(|ingredients_col| {
            for (ing_index, (resource_type, required)) in recipe.ingredients.iter().enumerate() {
                let have = player_inventory
                    .and_then(|inv| inv.resource_inventory.get(resource_type))
                    .copied()
                    .unwrap_or(0);

                let color = if have >= *required {
                    Color::srgb(0.0, 1.0, 0.0) // Green - have enough
                } else {
                    Color::srgb(1.0, 0.3, 0.3) // Red - not enough
                };

                ingredients_col.spawn((
                    Text::new(format!("{}: {}/{}", resource_type.name(), have, required)),
                    TextFont { font_size: 16.0, ..default() },
                    TextColor(color),
                    IngredientText(index, ing_index),
                ));
            }
        });

        // Craft button
        let button_color = if can_craft {
            Color::srgb(0.2, 0.5, 0.2) // Green-ish
        } else {
            Color::srgb(0.3, 0.3, 0.3) // Grey
        };

        row.spawn((
            Button,
            CraftButton(index),
            CraftButtonBg(index),
            Node {
                width: Val::Px(100.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(button_color),
        )).with_children(|btn| {
            btn.spawn((
                Text::new("Craft"),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });
    });
}

/// Check if player has enough resources to craft a recipe
fn check_can_craft(recipe: &Recipe, player_inventory: Option<&Inventory>) -> bool {
    let Some(inventory) = player_inventory else { return false };

    for (resource_type, required) in recipe.ingredients.iter() {
        let have = inventory.resource_inventory.get(resource_type).copied().unwrap_or(0);
        if have < *required {
            return false;
        }
    }
    true
}

pub fn despawn_crafting_ui(mut commands: Commands, query: Query<Entity, With<CraftingMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// =============================================================================
// BUTTON HANDLERS
// =============================================================================

pub fn handle_close_button(
    query: Query<&Interaction, (Changed<Interaction>, With<CloseButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Playing);
        }
    }
}

pub fn handle_craft_buttons(
    query: Query<(&Interaction, &CraftButton), Changed<Interaction>>,
    mut player_inventory: Query<&mut Inventory, With<Player>>
) {
    for (interaction, craft_button) in &query {
        if *interaction == Interaction::Pressed {
            let recipe_index = craft_button.0;
            let recipe = &ALL_RECIPES[recipe_index];
            info!("Craft button pressed for: {}", recipe.name);

            let Ok(mut inventory) = player_inventory.single_mut() else { return };

            if !has_resources(&inventory, recipe.ingredients) {
                return;
            }

            try_craft(&mut inventory, recipe);
        }
    }
}

/// Updates ingredient text and craft button colors based on current inventory
pub fn update_crafting_ui(
    player_query: Query<&Inventory, With<Player>>,
    mut ingredient_texts: Query<(&mut Text, &mut TextColor, &IngredientText)>,
    mut button_bgs: Query<(&mut BackgroundColor, &CraftButtonBg)>,
) {
    let Ok(inventory) = player_query.single() else { return };

    // Update ingredient texts
    for (mut text, mut color, IngredientText(recipe_idx, ing_idx)) in &mut ingredient_texts {
        let recipe = &ALL_RECIPES[*recipe_idx];
        let (resource_type, required) = &recipe.ingredients[*ing_idx];

        let have = inventory.resource_inventory.get(resource_type).copied().unwrap_or(0);

        **text = format!("{}: {}/{}", resource_type.name(), have, required);
        *color = if have >= *required {
            TextColor(Color::srgb(0.0, 1.0, 0.0)) // Green
        } else {
            TextColor(Color::srgb(1.0, 0.3, 0.3)) // Red
        };
    }

    // Update craft button colors
    for (mut bg_color, CraftButtonBg(recipe_idx)) in &mut button_bgs {
        let recipe = &ALL_RECIPES[*recipe_idx];
        let can_craft = has_resources(inventory, recipe.ingredients);

        *bg_color = if can_craft {
            BackgroundColor(Color::srgb(0.2, 0.5, 0.2)) // Green
        } else {
            BackgroundColor(Color::srgb(0.3, 0.3, 0.3)) // Grey
        };
    }
}
