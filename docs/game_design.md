# Drone Survival - Game Design Document

## Overview

A survival game where the player defends against waves of drones while managing resources, building defenses, and working toward escape via a crafted beacon.

---

## 1. Core Infrastructure Systems

### Message System

- Leverages Bevy's native [`Messages<T>`](https://docs.rs/bevy/0.18.0/bevy/ecs/message/index.html) with `MessageWriter`/`MessageReader`
- Define custom message types for game actions (damage, spawns, pickups, etc.)
- Decouples systems for flexible composition

### Game States

- Uses Bevy's [`States` and `SubStates`](https://docs.rs/bevy/0.18.0/bevy/state/index.html) for layered game modes
- Concurrent states (e.g., Playing + Building overlay, Paused)
- State transitions preserve game world; UI/menus layer on top

### Entity Factory

- Runtime spawning with blueprint/archetype definitions
- Supports entity composition from components
- Enables dynamic entity creation during gameplay

### Entity Lifecycle Manager

- Object pooling for frequently spawned entities (projectiles, effects)
- Timer-based automatic despawning
- Cleanup hooks for proper resource release

### Time Management

- Global pause functionality
- Time scaling (slow-mo, fast-forward)
- Per-system time control for UI independence

---

## 2. Entity Systems

### Player

- Movement, health, inventory
- Weapon handling, ability activation

### Drones (Enemies)

- Multiple drone types with varying behaviors
- Component-based composition for variants

### NPCs (Party Members)

- Recruitable allies with jobs and abilities
- AI-controlled behavior

### Projectiles

- Bullets, missiles, energy bolts
- Composable behaviors via components

### Structures

- Turrets, force field emitters, stations
- Health, power connections, repair state

---

## 3. Combat Systems

### Health/Damage

- Message-driven damage processing
- Armor and shield layers
- Damage types (kinetic, energy, etc.)

### Weapons

- Data-driven weapon definitions
- Configurable fire rates, patterns, projectile types
- Ammo consumption

### Projectiles

- Composable behaviors:
  - Homing
  - Piercing
  - Explosive (area damage)
  - Bouncing

### Targeting

- Reusable targeting logic for:
  - Player manual aim
  - NPC auto-aim
  - Turret tracking
- Priority systems (closest, weakest, strongest)

### Status Effects

- Buff/debuff system
- Stacking rules, duration timers
- Examples: slow, burn, shield boost, damage amp

---

## 4. Wave/Spawning Systems

### Wave Director

- Controls wave timing and composition
- Difficulty scaling over time
- Special wave events (boss waves, swarms)

### Spawn Points

- Dynamic spawn location selection
- Edge-of-screen, off-camera spawning
- Spawn point activation/deactivation

### Enemy Spawner

- Creates drone entities via Entity Factory
- Respects wave director composition
- Spawn rate throttling

---

## 5. Base Building Systems

### Placement System

- Grid-aligned building placement
- Placement validation (collision, resources)
- Preview/ghost display before confirming

### Structures

- Health and repair mechanics
- Power connections between structures
- Upgrade paths

### Turrets

- Auto-targeting using shared targeting system
- Configurable weapon types
- Ammo/energy consumption

### Force Fields

- Energy barriers between emitter pairs
- Blocks enemies and/or projectiles
- Power drain mechanics

---

## 6. Resource/Economy Systems

### Inventory

- Item storage for player and NPCs
- Stack management, capacity limits

### Resource Collection

- Enemy drops on death
- Pickup magnetism (auto-collect in radius)
- Manual collection option

### Crafting

- Recipe-based crafting system
- Categories:
  - Ammo
  - Medical supplies
  - Armor/shields
  - Weapons
  - Beacon (win condition)

### Resource Scarcity

- Track resource pressure metrics
- Informs NPC AI decisions (expedition risk vs. reward)
- Difficulty balancing input

---

## 7. NPC Management Systems

### Party System

- Recruit NPCs during gameplay
- Manage party composition and loadouts

### Job Assignment

- Assign NPCs to stations:
  - Crafting station
  - Repair duty
  - Combat support
  - Idle/follow

### NPC Abilities

- Component-based ability system:
  - Heal (restore player/NPC health)
  - Shoot (combat support)
  - Shield (temporary protection)

### NPC AI

- Behavior trees or utility AI
- Context-aware decision making
- Job-specific behaviors

### Expeditions

- Send NPCs on off-screen scavenging
- Resource rewards with time delay
- Death risk based on danger level

---

## 8. Progression Systems

### Win Condition

- Craft the beacon from collected resources
- Survive final extraction wave
- Victory screen with statistics

### Difficulty Scaling

- Time-based difficulty increase
- Wave composition escalation
- Resource scarcity pressure

### Statistics Tracking

- Kills by type
- Resources collected/spent
- NPCs recruited/lost
- Time survived
- End-game summary display

---

## 9. Presentation Systems

### ASCII Rendering

- Character-based sprites
- Color support (foreground/background)
- Animation frames
- Layer ordering (background, entities, effects, UI)

### UI Panels

- Health bar
- Party status
- Resource counts
- Wave indicator
- Menus (pause, inventory, crafting)

### Camera

- Follow player with smoothing
- World bounds clamping
- Screen shake effects

### Audio

- Sound effects for actions
- Background music
- Spatial audio for positional awareness

### Notifications

- In-game alert system
- Wave warnings, resource alerts, NPC status

---

## 10. Input Systems

### Input Mapping

- Rebindable controls
- Keyboard + potential gamepad support
- Context-sensitive actions

### Player Controller

- Movement input processing
- Action activation (shoot, interact, build)
- Mode-aware input handling

---

## Flexibility Principles

Addressing issues from previous `bevy_game` implementation:

| Previous Problem                  | Solution                                                             |
| --------------------------------- | -------------------------------------------------------------------- |
| Entities only at level load       | **Entity Factory** + pools enable runtime spawning anytime           |
| 3 hardcoded events                | **Bevy Messages** - define custom message types as needed            |
| Rigid state machine               | **States/SubStates** allow concurrent, non-destructive mode layering |
| Reactions only in BossFight state | Messages processed in **any state** via decoupled systems            |

### Design Principles

1. **Composition over inheritance** - Build entities from components
2. **Data-driven design** - Define weapons, enemies, waves in data files
3. **Message-driven communication** - Systems communicate via messages, not direct coupling
4. **Runtime flexibility** - Support spawning, registration, and changes during gameplay
5. **Reusable systems** - Targeting, damage, etc. shared across entity types

---

## MVP Feature Checklist

**Focus:** The core gameplay loop is the priority. MVP architecture should support future features (NPCs, base building, expeditions) without requiring rewrites—but those features are deferred until the core loop is solid.

### Core Loop (Priority)

- [Complete] Player movement and combat
- [Complete] Basic drone enemies with spawning
- [Complete] Wave system with progression
- [Complete] Resource drops and collection
- [ ] Basic crafting (ammo, beacon)
- [Partially complete] Win condition (beacon + final wave)
- [Complete] Game over / victory screens
- [ ] Inventory screen (view resources)
- [Complete] Music and sound effects
- [Complete] Audio settings (pause menu with volume controls)
- [ ] Audio preloading (load sounds at startup)

### Supporting Infrastructure

- [Following] Core infrastructure (messages, states, factory, time)
- [Following] ASCII rendering with UI
