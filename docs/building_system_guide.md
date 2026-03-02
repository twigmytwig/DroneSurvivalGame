# Building System & Extraction Beacon — Implementation Guide

## What We're Building

The player can already craft an Extraction Beacon, but it just prints a log message. We need a grid-based building system that lets the player physically place the beacon in the world, where it charges for 30 seconds and triggers Victory. This system is designed to later support walls, turrets, and other placeables.

## The Big Picture

The feature touches three areas of the game:

1. **A new `building` module** — grid math, structure spawning, build mode UI, beacon charging
2. **Inventory & crafting hooks** — so the beacon goes into a "placeable" inventory and can be placed from build mode
3. **Combat & lifecycle integration** — so enemies can damage structures, and structures clean up properly on death/game-over

The steps below are ordered so that each one builds on the last. Early steps create the data types and helpers. Middle steps wire up the actual gameplay. Late steps integrate with existing combat and cleanup systems.

---

## Step 1: Grid Coordinate Helpers — `src/building/grid.rs`

**What:** A `BuildGrid` resource (tracks which grid cells have structures in them) and a handful of pure math functions for converting between world coordinates and grid coordinates.

**Why this is first:** Everything in the building system revolves around a 32x32 pixel grid. The grid overlay needs to know where to draw lines. The ghost preview needs to snap to cell centers. Placement needs to check if a cell is occupied. Structure death needs to remove a cell from the grid. By defining the coordinate math and the `BuildGrid` resource up front, every later step can simply import and use them without circular dependencies.

**How it fits:** This file is pure data — no systems, no ECS queries. It's the shared vocabulary that grid.rs, build_mode.rs, placeable.rs, and damage.rs will all speak.

---

## Step 2: Placeable Types & Structure Spawning — `src/building/placeable.rs`

**What:** A `PlaceableType` enum (starting with just `ExtractionBeacon`), a `Structure` marker component, and a `spawn_structure()` function that creates a fully-formed structure entity in the world.

**Why this is second:** Before we can place anything, we need to define *what* can be placed and *how* to spawn it. `PlaceableType` is like `WeaponType` or `DroneType` — an enum that describes a category of thing. The `Structure` marker component (like `Enemy` or `Player`) lets us query for all structures in collision and cleanup systems. The `spawn_structure()` function is a factory that attaches all the right components (Transform, AsciiSprite, Health, Hitbox, etc.) so the entity is a proper game object from the moment it's placed.

**How it fits:** The build_mode placement system will call `spawn_structure()`. The collision systems will query for `Structure`. The death system will check for `Structure` to know when to update the grid. This file is the bridge between the grid data layer and the ECS entity layer.

---

## Step 3: Extraction Beacon Component — `src/building/extraction_beacon.rs`

**What:** An `ExtractionBeacon` component with a `Timer` inside it, and a `tick_beacon_charge` system that advances the timer each frame and transitions to `GameState::Victory` when it finishes.

**Why now:** This is a self-contained component + system pair. The beacon is the whole point of the building system — it's the win condition. We define it early so that `spawn_structure()` (from Step 2) can attach it to beacon entities. The timer starts ticking the moment the beacon is spawned, so there's no separate "activate" step. Survive 30 seconds with your beacon alive and you win.

**How it fits:** `spawn_structure()` adds an `ExtractionBeacon` component when the `PlaceableType` is `ExtractionBeacon`. The `tick_beacon_charge` system runs every frame during `GameState::Playing`, finds all `ExtractionBeacon` entities, ticks their timers, and fires the state transition on completion. If the beacon is destroyed before the timer finishes, the entity (and its component) are despawned, so the system simply stops finding it — no special cancellation logic needed.

---

## Step 4: Build Mode — `src/building/build_mode.rs`

**What:** The `BuildMode` resource (tracks whether we're in build mode and what's selected), the grid overlay visual, the ghost preview that follows the cursor, and the left-click placement system.

**Why now:** This is the player-facing heart of the feature. Steps 1-3 gave us the data layer (grid coordinates), the spawning logic (placeables + structures), and the win-condition component (beacon). Now we wire them into an interactive mode the player can toggle.

**What the systems do:**

- `toggle_build_mode` — B key toggles build mode on/off. Entering build mode picks the first placeable from the player's inventory. If they have nothing placeable, pressing B does nothing.
- `manage_grid_overlay` — When build mode turns on, spawn a grid of line sprites around the player (roughly 11x11 cells) so they can see the grid. When it turns off, despawn them.
- `update_grid_overlay_position` — The grid overlay follows the player each frame, snapped to grid alignment so lines always land on cell boundaries.
- `update_ghost_preview` — A semi-transparent copy of the structure's glyph follows the mouse cursor, snapped to the nearest grid cell. Green tint if the cell is free, red if occupied. This gives the player clear feedback before they click.
- `place_structure` — Left-click while in build mode: check cell is empty, check player has the item in inventory, consume it, call `spawn_structure()`, register the entity in `BuildGrid.occupied`. If the player runs out of that placeable type, automatically exit build mode.

**How it fits:** This is the only step where the player directly interacts with the building system. It ties together the grid (Step 1), the spawn factory (Step 2), and the inventory (Step 6) into a cohesive UX. It also sets up the `BuildMode` resource that Step 12 checks to disable shooting.

---

## Step 5: Module Root & Plugin — `src/building.rs`

**What:** The `mod` declarations for the four submodules above, `pub use` re-exports, and a `BuildingPlugin` that registers all resources and systems.

**Why now:** Steps 1-4 created the four submodule files, but nothing outside the building directory can see them yet. This file exposes the public API (`BuildGrid`, `BuildMode`, `Structure`, `PlaceableType`, `GridOverlay`, `GhostPreview`, `spawn_structure()`, etc.) and registers everything with Bevy's app builder. All systems are gated behind `run_if(in_state(GameState::Playing))`.

**How it fits:** After this step, the building module is self-contained and ready to be plugged into `main.rs`. The remaining steps modify *existing* files to integrate with it.

---

## Step 6: Placeable Inventory — `src/inventory/inventory_component.rs`

**What:** A new `placeable_inventory: HashMap<PlaceableType, u32>` field on the `Inventory` component, plus `add_placeable()`, `remove_placeable()`, and `has_placeable()` helper functions.

**Why now:** The building module is done, but the player has no way to *have* placeables. The existing `Inventory` component stores resources (a HashMap) and weapons (a fixed-size array). Placeables are a third category — they're stackable like resources, but they aren't raw materials. Adding them as a separate HashMap keeps the data model clean and matches the existing pattern.

**How it fits:** `add_placeable()` gets called by the crafting system (Step 7) when the player crafts a beacon. `remove_placeable()` and `has_placeable()` get called by the build_mode placement system (Step 4) when the player places a structure. This step is the bridge between crafting and building.

---

## Step 7: Wire Up Beacon Crafting — `src/crafting/recipe.rs`

**What:** Replace the `TODO` in the `CraftableItem::Beacon` branch of `try_craft()` with a call to `add_placeable(inventory, PlaceableType::ExtractionBeacon, 1)`.

**Why now:** This is a one-line change, but it's what completes the crafting → inventory → building pipeline. Right now, crafting a beacon consumes resources and logs a message. After this change, crafting a beacon puts it in the player's placeable inventory, where pressing B can select it and left-click can place it.

**How it fits:** This replaces the `info!("Beacon crafted! Handle win condition here.")` TODO that's been sitting in the codebase. The full flow is now: gather resources → craft beacon → beacon goes into placeable inventory → press B → left-click to place → beacon charges → Victory.

---

## Step 8: Structure Collision — `src/combat/collision.rs`

**What:** Two new collision systems: `enemy_projectile_hits_structure` and `enemy_collides_with_structure`. These are near-copies of the existing `enemy_projectile_hits_player` and `enemy_collides_with_player` systems, but they query for `Structure` instead of `Player`.

**Why now:** Structures exist in the world (Steps 1-5), and the beacon is the win condition. If enemies can't damage structures, there's no tension — the player would just place the beacon and wait. These collision systems make structures destructible, which creates the core gameplay loop: place the beacon, then defend it for 30 seconds.

**How it fits:** The systems follow the exact same pattern as the existing player collision systems. They query for enemy projectiles or exploding enemies, check circle overlap against structure hitboxes, and write `DamageEvent` messages. The existing `apply_damage` system (which just decrements health) handles the rest — no changes needed there.

---

## Step 9: Structure Death — `src/combat/damage.rs`

**What:** Modify the existing `apply_death` system to check if a dying entity has a `Structure` component. If so, remove it from `BuildGrid.occupied` before despawning.

**Why now:** Step 8 lets enemies damage structures. The existing `apply_damage` → `apply_death` pipeline already handles the health-reaches-zero → despawn flow. But if a structure dies without being removed from the grid, that cell stays "occupied" forever — the player couldn't rebuild there, and the grid state would be wrong.

**How it fits:** This is a small addition to an existing system. We add a `Query<&Structure>` and a `ResMut<BuildGrid>`, then before the generic `try_despawn()` call, check if the entity has `Structure` and clean up its grid cell. The rest of the death handling (resource drops from drones, game-over from player death) stays unchanged.

---

## Step 10: Game-Over Cleanup — `src/state/game_over.rs`

**What:** Extend `cleanup_game_entities` to also despawn all `Structure`, `GridOverlay`, and `GhostPreview` entities, and reset `BuildGrid` and `BuildMode` resources to their defaults.

**Why now:** The existing cleanup system despawns players, enemies, projectiles, and resource drops on game over. Without this change, structures would persist into the next game, the grid overlay could be left visible, and the occupied-cell map would still contain references to despawned entities. This step ensures a clean slate.

**How it fits:** It follows the exact same pattern as the existing cleanup — add a query, loop through, despawn. We also reset the two resources we added (`BuildGrid` and `BuildMode`) the same way the existing code resets `WaveState`.

---

## Step 11: Register the Plugin — `src/main.rs`

**What:** Add `mod building;` and `.add_plugins(building::BuildingPlugin)` to main.

**Why now:** This is the final wiring step. All the code exists, but the Bevy app doesn't know about it yet. One `mod` declaration and one `add_plugins` call brings everything online.

**How it fits:** This is where the building system becomes part of the game. The plugin registers all resources, initializes them to defaults, and schedules all building systems to run during `GameState::Playing`.

---

## Step 12: Disable Shooting in Build Mode — `src/player/shoot.rs`

**What:** Add `BuildMode` as a resource parameter to `player_shoot`. If `build_mode.selected.is_some()`, return early so the player doesn't fire.

**Why last:** This is a tiny quality-of-life fix, but it *depends* on the `BuildMode` resource existing (Step 5) and being registered (Step 11). Placing it last means we're integrating with a fully working system. Left-click in build mode should place a structure, not shoot a bullet. Without this guard, both would happen simultaneously.

**How it fits:** Build mode's `place_structure` system listens for left-click. The `player_shoot` system also listens for left-click. By having `player_shoot` bail out when build mode is active, we avoid the conflict cleanly without changing the input handling architecture.

---

## Summary: The Full Flow

```
Player gathers resources (existing)
        |
        v
Player opens crafting menu (existing)
        |
        v
Player crafts Extraction Beacon (Step 7 wires this up)
        |
        v
Beacon goes into placeable_inventory (Step 6)
        |
        v
Player presses B to enter build mode (Step 4)
        |
        v
Grid overlay appears, ghost preview follows cursor (Step 4)
        |
        v
Player left-clicks on empty cell (Step 4, shooting disabled by Step 12)
        |
        v
spawn_structure() creates beacon entity on grid (Steps 2, 3)
        |
        v
Beacon starts charging (30s timer — Step 3)
        |
        v
Enemies attack beacon (Step 8 collision, Step 9 death handling)
        |
        v
Beacon survives 30s → Victory!
Beacon destroyed → player can craft & try again
Game over → everything cleaned up (Step 10)
```
