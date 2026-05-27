# Offsets Directory

IL2CPP dumps and parsed offsets for each game patch.

## Automatic Updates

Run the offset dumper from main cheat repo:

```bash
cd ../f
cargo run --bin dump_offsets
```

This creates:
- `YYYY-MM-DD/offsets.json` - Machine-readable JSON
- `YYYY-MM-DD/offsets.rs` - Ready-to-use Rust code
- `YYYY-MM-DD/notes.md` - Dump info
- `latest.json` - Always current

## Format

Each patch gets a dated folder:

```
offsets/
├── 2026-05-27/
│   ├── offsets.json   # Full dump data
│   ├── offsets.rs     # Rust code (copy to src/)
│   └── notes.md       # What changed
└── latest.json        # Current offsets
```

## offsets.json Example

```json
{
  "timestamp": "2026-05-27 14:30:00",
  "game_version": "rust.client_12345",
  "base_addresses": {
    "LocalPlayer": "0x2F3A1C0",
    "ClientEntities": "0x2F3A1D0"
  },
  "base_player": {
    "health": "0x1F0",
    "maxHealth": "0x1F4",
    "playerModel": "0x4B0",
    "playerInput": "0x4D8"
  },
  "player_input": {
    "bodyAngles": "0x3C",
    "viewAngles": "0x44",
    "recoilAngles": "0x4C"
  },
  "player_model": {
    "newVelocity": "0x1D4",
    "transform": "0x30"
  }
}
```

## How to Update

### After Game Patch

1. Start Rust game
2. Join a server and spawn
3. Run: `cargo run --bin dump_offsets` (in main repo)
4. Offsets are auto-saved here
5. Rebuild cheat: `cargo build --release`

### Manual Verification

If auto-dump seems wrong:
- Open Cheat Engine
- Attach to RustClient.exe
- Search for your health value
- Verify offsets match dump
- Report issues if offsets are incorrect

## History

Keep old dumps for rollback:
- Game patches may break offsets
- Compare with previous versions
- Identify what changed
