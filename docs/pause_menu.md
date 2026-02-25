# Pause Menu with Settings

## Overview

A pause menu accessible via ESC during gameplay, with a settings submenu for audio volume control.

---

## Current State

- `GameState`: Loading, Playing, GameOver, Victory
- ESC currently only works in GameOver/Victory to restart
- No audio settings - sounds play at full volume
- UI pattern established: Node with BackgroundColor overlay + child Text

---

## Design Decisions

### Pause State Approach

**Using GameState::Paused** - The game already gates systems to Playing state. Adding a Paused state is clean because systems automatically stop via `run_if(in_state(Playing))`.

### Menu Structure

```
[Paused]
  - Resume
  - Settings >
      - Master Volume [slider]
      - SFX Volume [slider]
      - Music Volume [slider]
      - Back
  - Quit to Menu (goes to Loading)
```

---

## Implementation

### 1. AudioSettings Resource

**File:** `src/audio/settings.rs`

```rust
#[derive(Resource)]
pub struct AudioSettings {
    pub master: f32,  // 0.0 to 1.0
    pub sfx: f32,
    pub music: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            master: 0.5,
            sfx: 0.7,
            music: 0.4,
        }
    }
}

impl AudioSettings {
    pub fn effective_sfx(&self) -> f32 {
        self.master * self.sfx
    }
    pub fn effective_music(&self) -> f32 {
        self.master * self.music
    }
}
```

### 2. Update Audio Functions

**Files:** `src/audio/sfx.rs`, `src/audio/music.rs`

Add `settings: &AudioSettings` parameter and apply volume:

```rust
PlaybackSettings {
    volume: Volume::new(settings.effective_sfx()),
    ..PlaybackSettings::DESPAWN
}
```

### 3. Add GameState::Paused

**File:** `src/state/game_state.rs`

```rust
pub enum GameState {
    Loading,
    Playing,
    Paused,    // NEW
    GameOver,
    Victory,
}
```

### 4. Pause Menu UI

**File:** `src/state/paused.rs`

#### Components

- `PauseMenu` - root marker
- `PauseButton(PauseAction)` - enum variant for Resume/Settings/Quit
- `SettingsMenu` - settings panel marker
- `VolumeSlider { category }` - which volume (Master/SFX/Music)
- `SliderFill` - the colored fill bar

#### Systems

| System | Trigger | Description |
|--------|---------|-------------|
| `spawn_pause_menu` | OnEnter(Paused) | Creates pause menu UI |
| `despawn_pause_menu` | OnExit(Paused) | Removes pause menu |
| `handle_pause_input` | Update, Playing | ESC → Paused |
| `handle_resume_input` | Update, Paused | ESC → Playing |
| `handle_button_clicks` | Update, Paused | Button interactions |
| `handle_slider_click` | Update, Paused | Volume slider input |
| `update_slider_display` | Update, Paused | Sync fill width to value |

### 5. Register in StatePlugin

**File:** `src/state.rs`

```rust
// Paused state
.add_systems(OnEnter(GameState::Paused), paused::spawn_pause_menu)
.add_systems(OnExit(GameState::Paused), paused::despawn_pause_menu)
.add_systems(Update, (
    paused::handle_button_clicks,
    paused::handle_slider_click,
    paused::update_slider_display,
).run_if(in_state(GameState::Paused)))

// ESC to pause (during Playing)
.add_systems(Update, paused::handle_pause_input.run_if(in_state(GameState::Playing)))
```

---

## Files to Create/Modify

| File | Action |
|------|--------|
| `src/audio/settings.rs` | CREATE - AudioSettings resource |
| `src/audio.rs` | Add `mod settings` and re-export |
| `src/audio/sfx.rs` | Update play_sfx to use AudioSettings |
| `src/audio/music.rs` | Update play_music to use AudioSettings |
| `src/state/game_state.rs` | Add `Paused` variant |
| `src/state/paused.rs` | CREATE - pause menu UI and systems |
| `src/state.rs` | Add `mod paused`, register systems, init AudioSettings |
| `src/combat/collision.rs` | Update play_sfx calls |
| `src/state/game_over.rs` | Update play_music call |
| `src/state/victory.rs` | Update play_music call |

---

## UI Layout

### Main Pause Menu

```
+---------------------------+
|                           |
|         PAUSED            |
|                           |
|       [ Resume ]          |
|       [ Settings ]        |
|       [ Quit ]            |
|                           |
+---------------------------+
```

### Settings Submenu

```
+---------------------------+
|        SETTINGS           |
|                           |
|  Master:  [====----] 50%  |
|  SFX:     [=======-] 70%  |
|  Music:   [===-----] 40%  |
|                           |
|       [ Back ]            |
+---------------------------+
```

### Slider Implementation

Bevy has no built-in slider, so we create one with nested nodes:

```
SliderContainer (row layout)
├── Label ("Master:")
├── SliderBar (background, click target)
│   └── SliderFill (colored fill, width = value%)
└── ValueText ("50%")
```

Click anywhere on the slider bar to set the value (click_x / bar_width, clamped 0-1).

---

## Verification Checklist

- [ ] `cargo build` compiles
- [ ] Press ESC during gameplay → pause menu appears, game freezes
- [ ] Press ESC again or click Resume → game continues
- [ ] Click Settings → settings panel shows
- [ ] Click on volume sliders → values change
- [ ] New sounds reflect updated volume
- [ ] Click Back → returns to pause menu
- [ ] Click Quit → returns to Loading state
- [ ] Volume settings persist across pause/unpause

---

## Notes

- Volume changes apply immediately to new sounds
- Currently playing music won't change volume mid-track (would need AudioSink query)
- Slider uses click-to-set behavior
- Optional future enhancement: Add drag support for sliders
