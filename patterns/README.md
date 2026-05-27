# Weapon Recoil Patterns

Weapon recoil patterns for Rust game, used by the Read-Only Recoil Helper (v3.2).

## Directory Structure

```
patterns/
├── aob/              # AOB signatures for pattern scanning
│   └── signatures.txt
└── weapons/          # Weapon recoil patterns (NEW v3.2)
    ├── ak47.json
    ├── lr300.json
    └── mp5.json
```

## Weapon Pattern Format

Each weapon pattern is stored as JSON:

```json
{
  "weapon_id": 1,
  "weapon_name": "AK47",
  "fire_rate": 450.0,
  "max_pattern_length": 30,
  "recoil_pattern": [
    {"x": 0.0, "y": 4.0},
    {"x": -0.5, "y": 8.0},
    ...
  ],
  "notes": "Description of recoil behavior",
  "tips": "Tips for controlling this weapon"
}
```

### Fields:
- `weapon_id` - Unique weapon identifier (read from game memory)
- `weapon_name` - Display name
- `fire_rate` - Rounds per minute
- `max_pattern_length` - Pattern repeats after this many shots
- `recoil_pattern` - Array of {x, y} offsets per shot
  - `x` - Horizontal recoil (negative = left, positive = right)
  - `y` - Vertical recoil (always positive, goes up)
- `notes` - Optional description
- `tips` - Optional control tips

## Supported Weapons

### Assault Rifles
- **AK47** (`ak47.json`) - High recoil, hardest to control
- **LR300** (`lr300.json`) - Low recoil, easy to control
- **M39** - Semi-auto (TODO)

### SMGs
- **MP5** (`mp5.json`) - Fast fire, low recoil
- **Custom SMG** - Medium recoil (TODO)
- **Thompson** - High fire rate (TODO)

### More Weapons (TODO)
- M249
- M92 Pistol
- Python Revolver
- SPAS-12
- Pump Shotgun

## How to Get Patterns

### 1. From Memory (Runtime)
The Recoil Helper reads patterns directly from game memory:
```rust
let weapon = recoil_helper.read_weapon_from_memory(
    &process,
    player_addr,
    held_entity_offset,
    weapon_recoil_offset
);
```

### 2. From Community Sources
- UnknownCheats forums
- Cheat databases
- Pattern repositories

### 3. Manual Testing
1. Join empty server
2. Fire at wall
3. Measure bullet holes
4. Convert to JSON pattern

Example:
```
Shots 1-5: Mostly vertical (0, 4), (0, 8), (0, 12)...
Shots 6-15: Left-right sway (-2, 15), (2, 16), (-2, 17)...
Shots 16-30: Smaller sway pattern
```

## Pattern Accuracy

Patterns may vary slightly depending on:
- **Game version** - Updates may change recoil
- **Attachments** - Muzzle brake, compensator, etc.
- **Weapon condition** - Degradation affects recoil
- **Server settings** - Some servers modify recoil

**Recommended:** Update patterns after major game updates.

## Usage in Recoil Helper

Enable in `config.toml`:
```toml
recoil_helper_enabled = true
recoil_compensation_strength = 0.8
recoil_show_weapon_info = true
```

The Recoil Helper will:
1. Detect equipped weapon
2. Load pattern from JSON or memory
3. Track current shot count
4. Calculate compensation offset
5. Display visual guide (green crosshair)

## Adding New Weapons

1. Find weapon ID in game memory
2. Dump recoil pattern (fire at wall + measure)
3. Create JSON file in `weapons/` directory
4. Test with Recoil Helper
5. Adjust `compensation_strength` if needed

Example:
```bash
# Fire at wall, measure, create pattern
nano weapons/m249.json

# Test
cargo run --release
# Equip M249 in game, check if pattern detected
```

## Pattern Validation

Check pattern accuracy:
```rust
// In recoil_helper.rs tests
#[test]
fn test_ak47_pattern() {
    let pattern = load_pattern("weapons/ak47.json");
    assert_eq!(pattern.weapon_id, 1);
    assert_eq!(pattern.recoil_pattern.len(), 30);
    assert!(pattern.fire_rate > 0.0);
}
```

## Notes

- Patterns are **approximate** - Rust has slight randomness
- Use `compensation_strength = 0.8` for natural feel
- Update patterns after game updates
- Test on alt account first

## Related Docs

- `RECOIL_HELPER.md` - Recoil helper usage guide
- `config.toml.example` - Configuration options
- Main cheat repo: https://github.com/lukasluk998/f
