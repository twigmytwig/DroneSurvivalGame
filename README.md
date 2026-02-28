# Drone Survival

An ASCII-based survival game built with Rust and Bevy. Defend against waves of drones, collect resources, craft weapons, and build an extraction beacon to escape.

## How to Run

### Prerequisites

1. **Install Rust**
   ```bash
   # Linux/macOS
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # Windows
   # Download and run rustup-init.exe from https://rustup.rs
   ```

2. **Verify installation**
   ```bash
   rustc --version
   cargo --version
   ```

### Running the Game

```bash
# Clone the repository
git clone <repository-url>
cd drone_survival

# Run in debug mode
cargo run

# Run in release mode (better performance)
cargo run --release
```

## Features

### Combat
- Player movement and shooting
- Multiple drone enemy types with unique behaviors
- Wave-based progression with increasing difficulty
- Projectile system with configurable weapons

### Inventory System
- Resource inventory with stacking
- Weapon inventory
- Hotbar UI displaying collected items and weapons

### Resource Drops & Pickups
- Enemies drop resources on death (Scrap Metal, Circuitry, Drone Weapon Parts)
- Magnetic pickup system for automatic collection
- Resources have lifetime timers with visual blinking before despawn

### Crafting Menu
- Press **Tab** to open crafting menu
- Craft weapons (Shotgun) and items (Extraction Beacon)
- Real-time ingredient display showing current vs required resources
- Visual feedback for craftable recipes

### Audio System
- Sound effects for combat actions
- Background music
- Audio preloading during startup
- Volume controls in pause menu (Master, SFX, Music)

### Game States
- Loading screen with asset preloading
- Pause menu with resume, settings, and quit options
- Game over and victory screens
- Wave countdown and progress tracking
