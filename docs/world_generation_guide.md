# Procedural World Generation — Implementation Guide

## What We're Building

The game currently has no world — just empty black space. We need a chunk-based procedural world where the player walks through a post-apocalyptic landscape of dead forests, ruins, and wasteland. Features are multi-tile ASCII art entities (trees, buildings, rubble) that block movement. The black background IS the ground — we only spawn the interesting stuff.

## The Big Picture

The feature touches three areas:

1. **A new `world` module** — feature templates, chunk loading/unloading, noise-based generation, terrain collision grid
2. **Player movement** — so terrain blocks the player
3. **Building & cleanup integration** — so you can't build on terrain, and game-over resets the world

The steps below are ordered so each one builds on the last. Early steps define data types. Middle steps wire up chunk lifecycle. Late steps add generation and integrate with existing systems.

---

## Step 1: Feature Templates — `src/world/types.rs`

**What:** A `FeatureTemplate` struct that defines how a multi-tile feature looks and what grid cells it occupies. A `FeatureType` enum for the different kinds of features. A `BiomeType` enum for the biome categories.

**Why this is first:** Everything in world generation revolves around features. The chunk system needs to know what to spawn. The terrain grid needs to know which cells to mark. The renderer needs the glyph. By defining templates up front, every later step can just look up a template and use it.

**Key structs:**

```rust
pub struct FeatureTemplate {
    pub glyph: &'static str,              // multi-line ASCII art
    pub color: Color,
    pub font_size: f32,
    pub footprint: &'static [(i32, i32)], // grid cell offsets from anchor
    pub blocks_movement: bool,
}
```

`FeatureType` is an enum (`DeadTree`, `RuinWall`, `Rubble`, etc.) with a `template()` method that returns the `FeatureTemplate` for each variant — same pattern as `PlaceableConfig::from_type()`.

`BiomeType` is an enum (`DeadForest`, `Ruins`, `Wasteland`, `Scrapyard`, `Hollow`) — pure data, no methods needed yet.

**How it fits:** This file is pure data — no systems, no ECS queries. It's the shared vocabulary that every other world file will import.

---

## Step 2: Terrain Grid — `src/world/terrain_grid.rs`

**What:** A `TerrainGrid` resource with an `impassable: HashSet<(i32, i32)>` that tracks which grid cells are blocked by terrain features.

**Why this is second:** The terrain grid is the bridge between world generation and gameplay. When a feature spawns, its footprint cells get added here. When the player tries to move, this is what gets checked. By defining it early, the chunk system (Step 3) can populate it and the movement system (Step 8) can query it.

**Helper methods:**
- `mark_impassable(&mut self, cells: &[(i32, i32)])` — bulk-add cells
- `clear_cells(&mut self, cells: &[(i32, i32)])` — bulk-remove cells (for chunk despawn)
- `is_blocked(&self, cell: (i32, i32)) -> bool` — single-cell passability check

**How it fits:** Chunk spawn adds cells. Chunk despawn removes cells. Player movement and building placement check cells. Derive `Default` so it starts empty.

---

## Step 3: Chunk Manager & Lifecycle — `src/world/chunk.rs`

**What:** The `ChunkManager` resource that tracks which chunks are loaded, a `ChunkMember` component for tagging entities to their chunk, a `TerrainFeature` component that stores a feature's absolute footprint cells, and the systems that load/unload chunks as the player moves.

**Why now:** We need the infrastructure to spawn and despawn groups of entities as the player explores. Without chunks, we'd have to spawn the entire world at once (impossible for an infinite world) or do something ad-hoc.

**Key types:**
- `ChunkCoord(i32, i32)` — a chunk's position in chunk-space (not world-space)
- `CHUNK_SIZE: i32 = 16` — 16x16 tiles per chunk = 1024x1024 world units
- `ChunkManager` resource — `loaded_chunks: HashMap<(i32,i32), Vec<Entity>>`, `load_radius: i32` (3 = 7x7 grid of chunks), `player_chunk: (i32, i32)`
- `ChunkMember(pub i32, pub i32)` component — tags an entity to its chunk
- `TerrainFeature { pub footprint_cells: Vec<(i32, i32)> }` component — the absolute grid cells this entity occupies

**Systems:**
- `update_player_chunk` — converts player world position to chunk coord, updates `ChunkManager.player_chunk`
- `load_chunks` — compares desired chunks (within `load_radius` of player) against `loaded_chunks`. For any missing chunk, generates it (calls into Step 5's generator) and spawns feature entities. Each entity gets `Transform`, `AsciiSprite`, `ChunkMember`, and `TerrainFeature`. Marks footprint cells in `TerrainGrid`.
- `unload_chunks` — finds loaded chunks outside `load_radius`. For each entity in the chunk, reads its `TerrainFeature` and clears those cells from `TerrainGrid`. Then despawns the entity.

**How it fits:** This is the engine that makes the infinite world work. Step 5 (generation) tells it *what* to spawn. This step handles *when* to spawn/despawn it and keeps the terrain grid in sync.

---

## Step 4: World Seed — `src/world/generation.rs` (part 1)

**What:** A `WorldSeed(pub u64)` resource and the skeleton of a `WorldGenerator` struct.

**Why now:** Before we can generate anything, we need a seed so the world is deterministic (same seed = same world). This is a tiny step — just the resource and an empty struct. We'll fill in the actual generation logic in Step 5.

**How it fits:** `WorldSeed` gets initialized when the game starts (random or hardcoded for testing). `WorldGenerator` will be constructed from it.

---

## Step 5: Noise-Based Generation — `src/world/generation.rs` (part 2)

**What:** The full `WorldGenerator` with noise sampling. Given a chunk coordinate, it produces a list of `(grid_x, grid_y, FeatureType)` placements.

**Why now:** Steps 1-3 gave us the data types, the terrain grid, and the chunk lifecycle. Now we need the actual algorithm that decides what goes where.

**How it works:**
1. Create 2 noise layers from the seed: one low-frequency (biome), one higher-frequency (feature density)
2. For each tile in the chunk, sample biome noise → map to `BiomeType`
3. Walk through candidate positions, sample density noise → decide if a feature spawns
4. Pick a `FeatureType` based on biome (e.g. DeadForest → DeadTree, Ruins → RuinWall)
5. Look up the `FeatureTemplate`, get its footprint offsets
6. Check ALL footprint cells against a local occupancy set — if any cell is taken, skip
7. If all cells free, add the placement and mark those cells as occupied
8. Skip any features whose footprint overlaps with ~5 tiles of world origin (spawn clearing)

**Start with 2-3 biomes** (DeadForest, Ruins, Wasteland). Add more later.

**How it fits:** `load_chunks` (Step 3) calls `WorldGenerator::generate_chunk()` to get placements, then spawns entities for each one. The generation is pure computation — no ECS, no side effects. It returns data, the chunk system spawns from it.

---

## Step 6: Module Root & Plugin — `src/world.rs`

**What:** The `mod` declarations for the four submodules, `pub use` re-exports, and a `WorldPlugin` that registers all resources and systems.

**Why now:** Steps 1-5 created the submodule files, but nothing outside the world directory can see them yet. This file exposes the public API and registers everything with Bevy.

**Systems registration:** All systems gated behind `run_if(in_state(GameState::Playing))`.

**How it fits:** After this step, the world module is self-contained and ready to plug into `main.rs`.

---

## Step 7: Register the Plugin — `src/main.rs`

**What:** Add `mod world;` and `.add_plugins(world::WorldPlugin)` to main.

**Why now:** One-line wiring step. After this, you should be able to `cargo run` and see features spawning around you as you walk.

**Test it:** Walk around. Features should appear in the distance and disappear behind you. Different areas should look different (biome variation). Features should be multi-line ASCII art, not single characters.

---

## Step 8: Terrain Blocks Player Movement — `src/player/movement.rs`

**What:** Before applying player movement, check `TerrainGrid.is_blocked()` for the grid cell the player would move into. If blocked, don't apply movement in that direction.

**Why now:** Features are visible (Step 7) but the player walks right through them. This step makes trees and ruins solid.

**How it works:** Convert the player's *target* position (current pos + movement delta) to grid coords using `world_to_grid()`. Check `terrain_grid.is_blocked()`. If blocked, zero out that axis of movement (so the player can still slide along walls).

**How it fits:** Small addition to one existing system. Uses `TerrainGrid` (Step 2) and `world_to_grid()` from `building::grid`.

---

## Step 9: Building Blocked on Terrain — `src/building/build_mode.rs`

**What:** In `place_structure`, reject placement if any footprint cell is in `TerrainGrid.impassable`. In `update_ghost_preview`, show red tint on impassable tiles.

**Why now:** Without this, the player could place buildings inside trees. Small change — just add a `Res<TerrainGrid>` parameter and one extra check.

**How it fits:** Mirrors the existing "is cell occupied in BuildGrid?" check. Now we check both BuildGrid (player-placed structures) and TerrainGrid (world features).

---

## Step 10: Game-Over Cleanup — `src/state/game_over.rs`

**What:** Extend `cleanup_game_entities` to despawn all entities with `ChunkMember`. Reset `ChunkManager` and `TerrainGrid` to defaults.

**Why now:** Without this, terrain persists across games and the chunk manager thinks chunks are still loaded. Same pattern as the existing cleanup for structures, grid overlay, etc.

**How it fits:** Add a query for `ChunkMember`, loop + despawn. Insert default resources. Follows the exact pattern already used for `Structure`, `GridOverlay`, `GhostPreview`, `BuildGrid`, `BuildMode`.

---

## Summary: The Full Flow

```
Game starts → WorldSeed created (random)
    |
    v
Player spawns at origin (spawn clearing = no features nearby)
    |
    v
Player moves → update_player_chunk detects chunk change
    |
    v
load_chunks generates missing chunks via WorldGenerator
    |
    v
Features spawn as multi-tile ASCII entities
TerrainGrid marks their footprint cells as impassable
    |
    v
Player walks into tree → movement blocked (Step 8)
Player tries to build on tree → placement rejected (Step 9)
    |
    v
Player walks away → unload_chunks despawns distant features
TerrainGrid clears their footprint cells
    |
    v
Game over → cleanup despawns all ChunkMember entities
           → resets ChunkManager + TerrainGrid
```

## Files to Create
- `src/world.rs` — module root (Step 6)
- `src/world/types.rs` — BiomeType, FeatureType, FeatureTemplate (Step 1)
- `src/world/chunk.rs` — ChunkManager, lifecycle systems (Step 3)
- `src/world/generation.rs` — WorldGenerator, noise (Steps 4-5)
- `src/world/terrain_grid.rs` — TerrainGrid resource (Step 2)

## Files to Modify
- `Cargo.toml` — add `noise = "0.9"` (Step 5)
- `src/main.rs` — add `mod world;` + plugin (Step 7)
- `src/player/movement.rs` — terrain collision check (Step 8)
- `src/building/build_mode.rs` — terrain passability check (Step 9)
- `src/state/game_over.rs` — cleanup (Step 10)

## Existing Code to Reuse
- `world_to_grid()` / `grid_to_world()` from `src/building/grid.rs` — same 64-unit grid
- `AsciiSprite` from `src/ascii_sprite.rs` — already handles multi-line text rendering
- `GameState::Playing` from `src/state/game_state.rs` — for system gating
- Cleanup pattern from `src/state/game_over.rs` — query + despawn loop

## Verification
1. `cargo run` — walk around, see multi-tile features loading around you
2. Different areas should have different biome features (trees vs ruins vs rubble)
3. Walking into trees/ruins should block movement
4. Building placement should be blocked on terrain features
5. Game over → restart should reset the world cleanly
6. FPS should stay above 60
