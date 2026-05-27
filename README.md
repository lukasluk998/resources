# Resources Repository

Automatically dumped offsets, patterns, and tools for Rust game cheat development.

## Contents

### `/offsets/` - Game Offsets
Automatically dumped offsets from each game patch:
- `YYYY-MM-DD/` - Dated offset dumps
  - `offsets.json` - Machine-readable offset data
  - `offsets.rs` - Ready-to-use Rust code
  - `notes.md` - Dump info and usage notes
- `latest.json` - Always points to most recent dump

### `/patterns/` - AOB Patterns
Signature patterns for anti-cheat resistant scanning:
- `weapons/` - Weapon recoil patterns
- `aob/` - Memory signatures
- `entities/` - Entity structure patterns

### `/dumps/` - Full IL2CPP Dumps
Complete IL2CPP dump outputs (if manually dumped):
- `il2cpp/YYYY-MM-DD/` - Full dumps per patch

### `/tools/` - Helper Tools
Utilities and scripts:
- Runtime dumpers
- Offset parsers
- Pattern generators

## Usage

### Automatic Offset Dumping (Recommended)

The main cheat repo includes an **automatic runtime offset dumper**:

```bash
# 1. Start Rust game and join a server
# 2. Run the dumper tool
cd ../f
cargo run --bin dump_offsets

# This will:
# - Attach to RustClient.exe
# - Scan for all offsets automatically
# - Save to this resources repo
# - Update src/offsets.rs in main cheat
# - Optionally commit to git
```

**No need for Il2CppDumper!** The runtime dumper finds everything automatically.

### Manual Il2CppDumper Method (Backup)

If runtime dumper fails, you can still use Il2CppDumper manually:

```bash
# 1. Download Il2CppDumper
# https://github.com/Perfare/Il2CppDumper/releases

# 2. Extract game files
# GameAssembly.dll from: SteamLibrary\steamapps\common\Rust\
# global-metadata.dat from: SteamLibrary\steamapps\common\Rust\RustClient_Data\il2cpp_data\Metadata\

# 3. Run Il2CppDumper
Il2CppDumper.exe GameAssembly.dll global-metadata.dat

# 4. Check dump.cs for offsets:
# - BasePlayer class → health, playerModel, playerInput
# - PlayerInput class → recoilAngles, bodyAngles, viewAngles
# - PlayerModel class → newVelocity, transform

# 5. Add to this repo
mkdir offsets/YYYY-MM-DD
cp dump.cs offsets/YYYY-MM-DD/
# Update offsets/latest.json
```

### Copy Offsets to Main Cheat

```bash
# After dumping, copy to main cheat
cp resources/offsets/latest/offsets.rs ../f/src/offsets.rs

# Rebuild cheat
cd ../f
cargo build --release
```

## Structure

```
resources/
├── offsets/
│   ├── 2026-05-27/          # Auto-dumped offsets
│   │   ├── offsets.json     # JSON format
│   │   ├── offsets.rs       # Rust code
│   │   └── notes.md         # Dump info
│   └── latest.json          # Symlink to latest
├── patterns/
│   ├── weapons/
│   │   ├── ak47.json
│   │   ├── lr300.json
│   │   └── ...
│   ├── aob/
│   │   ├── localplayer.txt
│   │   └── ...
│   └── entities/
│       └── baseplayer.reclass
├── dumps/
│   └── il2cpp/
│       └── 2026-05-27/
│           ├── dump.cs
│           └── script.json
└── tools/
    └── offset_updater.py
```

## Why Runtime Dumper?

**Advantages over Il2CppDumper:**
- ✅ No manual extraction needed
- ✅ Works directly from running game
- ✅ Finds actual runtime values (more accurate)
- ✅ Auto-generates Rust code
- ✅ One-click update after patches
- ✅ No need to close game

**When to use Il2CppDumper instead:**
- Game has aggressive anti-debug
- You don't have the game running
- You need full class definitions

## Updating After Game Patch

1. **Start Rust** and join a server
2. **Run dumper:** `cargo run --bin dump_offsets` in main cheat repo
3. **Rebuild cheat:** `cargo build --release`
4. **Done!** Offsets are updated

The dumper automatically:
- Finds new offsets
- Saves to this repo (dated folder)
- Updates `latest.json`
- Updates `src/offsets.rs` in main cheat
- Backs up old offsets
- Optionally commits to git

## Main Cheat Repo

https://github.com/lukasluk998/f

## Disclaimer

Educational purposes only. Use at your own risk.
