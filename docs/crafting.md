# Crafting System Implementation Plan

## Overview

Player presses Tab to open crafting menu. Shows available recipes. Player can craft items if they have the required resources. Crafted items go to inventory.

**MVP Recipes:**
- Shotgun: 2 Circuitry + 2 Scrap Metal
- Extraction Beacon: TBD ingredients (win condition)

**File Structure:**
```
src/crafting.rs          # Module root - declares submodules, re-exports
src/crafting/
    recipe.rs            # Recipe struct, CraftableItem enum, recipe constants
    crafting_ui.rs       # UI spawning, button handlers
```

---

## Implementation Steps

### 1. Define Data Structures

**File:** `src/crafting/recipe.rs`

```rust
enum CraftableItem {
    Weapon(WeaponType),
    Beacon,
}

struct Recipe {
    name: &'static str,
    ingredients: &'static [(ResourceType, u32)],
    output: CraftableItem,
}
```

Define the two recipes as constants.

**File:** `src/crafting.rs`

```rust
mod recipe;
mod crafting_ui;

pub use recipe::*;
pub use crafting_ui::*;
```

---

### 2. Expand Inventory

**File:** `src/inventory/inventory_component.rs`

Add to the Inventory struct:
```rust
weapon_inventory: HashSet<WeaponType>,
has_beacon: bool,
```

---

### 3. Add Crafting Game State

**File:** `src/state/game_state.rs`

- Add `GameState::Crafting`
- Update `InGame` computed state to include Crafting (so wave state persists)

---

### 4. Add Tab Key to Open Crafting

**File:** `src/state.rs` or input handling

- In Playing state, Tab key transitions to `GameState::Crafting`
- In Crafting state, Tab or Escape returns to `GameState::Playing`

---

### 5. Build Crafting UI

**File:** `src/crafting/crafting_ui.rs`

- `spawn_crafting_ui` - runs on `OnEnter(GameState::Crafting)`
- `despawn_crafting_ui` - runs on `OnExit(GameState::Crafting)`
- Display list of recipes with:
  - Recipe name
  - Required ingredients (show what player has vs what's needed)
  - "Craft" button (disabled if insufficient resources)
  - "Close" button to exit menu

---

### 6. Implement Craft Button Logic

**File:** `src/crafting/crafting_ui.rs`

`handle_craft_button` system - when player clicks Craft:
1. Check player has required resources
2. Deduct resources from `resource_inventory`
3. Add item to appropriate inventory:
   - Weapon → `weapon_inventory.insert(weapon_type)`
   - Beacon → `has_beacon = true`
4. Play craft sound effect (optional)

---

### 7. Hook Up Beacon Win Condition

**File:** `src/state/victory.rs` or wave system

When `has_beacon` becomes true:
- Trigger final extraction wave, OR
- Immediately trigger victory

(Decide: does crafting beacon = instant win, or does it start a final wave?)

---

### 8. Register Systems

**File:** `src/state.rs`

```rust
// Tab key to open/close crafting
.add_systems(Update, toggle_crafting.run_if(in_state(GameState::Playing)))
.add_systems(Update, close_crafting.run_if(in_state(GameState::Crafting)))

// Crafting UI
.add_systems(OnEnter(GameState::Crafting), spawn_crafting_ui)
.add_systems(OnExit(GameState::Crafting), despawn_crafting_ui)
.add_systems(Update, handle_craft_button.run_if(in_state(GameState::Crafting)))
```

---

## Open Questions

- Beacon crafting = instant win or triggers final wave?
- What are the beacon ingredients?
- Should we show a "crafted!" feedback message?
