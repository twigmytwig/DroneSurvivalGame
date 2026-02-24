# Resource System Plan

This document outlines the design for the resource drop, pickup, and storage systems.

---

## Resource Types

Three resource types for MVP:

| Resource         | Glyph | Color  | Description                     |
|------------------|-------|--------|---------------------------------|
| Scrap            | `#`   | Gray   | Common building material        |
| Drone Gun Parts  | `%`   | Orange | Salvaged weapon components      |
| Circuitry        | `&`   | Cyan   | Electronic components           |

Resources are **specialized** - crafting recipes require specific resource types (e.g., shotgun needs 3 Scrap + 1 Drone Gun Parts).

---

## Drop System

### DropTable (Separate from DroneConfig)

Drop rules are defined in a **separate DropTable system**, not embedded in DroneConfig. This keeps drone behavior and loot definitions decoupled, and allows reuse for future loot sources (crates, bosses, etc.).

Each drone type is linked to a drop table that defines:
- Which resource types can drop
- Quantity range per resource type
- Weighting (probability distribution)

### Drop Rules

| Rule | Value |
|------|-------|
| Drop chance | 100% (always drops something) |
| Quantity | Random, weighted toward lower amounts |
| Multi-type | One drone can drop multiple resource types |
| Spread | Small random offset from death position (items don't stack visually) |

### Example Drop Table

```
Chaser drone:
  - Scrap: 1-2 (weighted low)

Shooter drone:
  - Scrap: 1-2 (weighted low)
  - Drone Gun Parts: 0-1 (50% chance of 1)
```

---

## Pickup System

### Two Modes (Player Setting)

| Mode | Behavior |
|------|----------|
| Magnetism (default) | Items drift toward player within short radius, auto-collected |
| Manual | Player presses E to pick up nearby items |

Magnetism is the default for MVP since inventory is unlimited. The setting becomes more relevant when inventory limits are added (players may want to be selective about what they pick up).

---

## Storage

### MVP
- **Unlimited capacity**
- Simple resource counts on player (e.g., `Scrap: 12, Drone Gun Parts: 3, Circuitry: 5`)

### Future (Post-MVP)
- Limited inventory capacity
- Base storage (drop off resources at constructed base)
- Inventory management decisions

---

## Drop Lifetime

| Aspect | Value |
|--------|-------|
| Despawn timer | 30 seconds |
| Warning | Flash/blink animation when 10 seconds or less remaining |

This adds tension - player must decide whether to grab loot or keep fighting.

---

## Crafting (Context)

Resources will be used for crafting. Example recipes:

| Item | Recipe |
|------|--------|
| Shotgun | 3 Scrap + 1 Drone Gun Parts |
| Extraction Beacon (win condition) | 10 Scrap + 10 Drone Gun Parts + 10 Circuitry |

Crafting system details are separate from this plan.

---

## Implementation Order

1. **Resource types** - Define enum/struct for resource types
2. **DropTable system** - Define drop tables, link to drone types
3. **Drop spawning** - Spawn resource entities when drone dies
4. **Drop visuals** - ASCII glyphs with colors, spread offset
5. **Drop lifetime** - Despawn timer with flash warning
6. **Player inventory** - Resource storage (simple counts)
7. **Pickup system** - Magnetism + manual modes
8. **Pickup setting** - Toggle between modes

---

## File Structure (Proposed)

```
src/
  resources.rs       # Module declarations, re-exports
  resources/
    resource.rs      # Resource enum, ResourceDrop component
    drop_table.rs    # DropTable definitions per drone type
    pickup.rs        # Pickup systems (magnetism, manual)
    inventory.rs     # Player inventory resource
```
