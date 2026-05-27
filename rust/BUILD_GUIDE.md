# Build Guide - v3.4

## Prerequisites

### 1. Install Rust
```powershell
# Download and run rustup-init.exe from:
https://rustup.rs/

# Or direct link:
https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe

# Run installer and select default options
# This installs:
# - rustc (Rust compiler)
# - cargo (package manager)
# - rustup (toolchain manager)
```

### 2. Install Visual Studio Build Tools (if not already installed)
```powershell
# Download from:
https://visualstudio.microsoft.com/downloads/

# Install "Desktop development with C++" workload
# Or just "MSVC v142 - VS 2019 C++ x64/x86 build tools"
```

### 3. Verify Installation
```powershell
rustc --version
cargo --version

# Should output something like:
# rustc 1.77.0 (aedd173a2 2024-03-17)
# cargo 1.77.0 (3fe68eabf 2024-02-29)
```

## Building

### Method 1: Random Name Build (RECOMMENDED)
```powershell
# Windows PowerShell
.\rename_build.ps1

# Output: discord_helper_7e9d2f8a.exe (random name each build)
```

### Method 2: Manual Build
```powershell
# Standard release build
cargo build --release

# Output: target\release\rust-game-cheat.exe
```

### Method 3: Custom Name
```powershell
# Edit build.rs and change PROCESS_NAME
# Then build normally
cargo build --release
```

## Configuration

### 1. Copy Example Config
```powershell
copy config.toml.example config.toml
notepad config.toml
```

### 2. Choose Preset
```toml
# Uncomment ONE preset in config.toml:

# For main account (safest):
preset = "ultra_safe"

# For alt account:
# preset = "balanced"

# For burner account:
# preset = "rage"
```

### 3. Enable v3.4 Features
```toml
# Make sure these are enabled:
behavioral_limiter_enabled = true
gradual_unlock_enabled = true
anti_debug_enabled = true
```

## Running

### 1. Start Rust Game First
```
Launch Rust normally
Wait for main menu
```

### 2. Run Cheat
```powershell
# Run as Administrator (if using kernel driver)
.\discord_helper_7e9d2f8a.exe

# Or without admin (read-only mode)
.\discord_helper_7e9d2f8a.exe
```

### 3. Expected Output
```
╔══════════════════════════════════════════════╗
║   Rust EAC Bypass Cheat v3.4 - 2026         ║
║   Ultimate Safety Package                   ║
╚══════════════════════════════════════════════╝

[+] License Type: Free
[+] Configuration loaded:
    Mode: Legit
    ESP: ✓
    No Recoil: Macro
    ...

[+] v3.4 Ultimate Safety Features:
    Behavioral Limiter: ✓ ENABLED
    Gradual Unlock: ✓ ENABLED
    Anti-Debug: ✓ ENABLED
    ...

╔════════════════════════════════════════════╗
║     GRADUAL UNLOCK STATUS                  ║
╚════════════════════════════════════════════╝

Days since first run: 0 (0 hours)
Total runs: 1

Feature Unlock Status:
  ESP: ✓ UNLOCKED (max distance: 50m / 300m)
       ○ Basic info only (distance)
  Recoil Helper: ○ Unlocks in 5 days
  Optimizations: ○ Unlocks in 1 days

[+] Waiting for RustClient.exe...
[+] Found RustClient.exe (PID: 12345)
[+] GameAssembly.dll: 0x7FF6A0000000
[+] LocalPlayer found!
[+] Cheat running... Press Ctrl+C to exit
```

## Troubleshooting

### "cargo: command not found"
```powershell
# Restart PowerShell after installing Rust
# Or add to PATH manually:
$env:Path += ";$env:USERPROFILE\.cargo\bin"
```

### "Failed to find process"
```
- Make sure Rust game is running
- Run cheat as Administrator
- Check process name in Task Manager
```

### "Failed to read memory"
```
- Game offsets need updating
- See find_offsets.md for instructions
- Or wait for offset update
```

### "Debugger detected"
```
- Close x64dbg, IDA Pro, Cheat Engine
- Close Process Hacker, Process Explorer
- Disable anti-virus temporarily
- Set anti_debug_enabled = false in config (not recommended)
```

### Compilation Errors
```powershell
# Update Rust
rustup update

# Clean build
cargo clean
cargo build --release

# Check for missing dependencies
cargo check
```

## Build Optimization

### Smaller Binary
```toml
# Already in Cargo.toml:
[profile.release]
opt-level = 3        # Maximum optimization
lto = true           # Link-time optimization
codegen-units = 1    # Single codegen unit
strip = true         # Strip symbols
panic = "abort"      # Smaller panic handler
```

### Faster Compilation
```toml
# For development builds:
[profile.dev]
opt-level = 0
```

## Security Notes

### 1. Random Process Name
- **CRITICAL**: Use `rename_build.ps1` for every build
- Never use same .exe name twice
- Process name is signature for detection

### 2. String Obfuscation
- Strings are encrypted at compile-time
- No "RustClient.exe" or "GameAssembly.dll" in binary
- Static analysis won't find cheat strings

### 3. Anti-Debug
- Detects debuggers on startup
- Background monitoring during runtime
- Auto-exit if analysis detected

### 4. Gradual Unlock
- Features unlock over 12 days
- Mimics natural player improvement
- **DO NOT DISABLE** for main account

### 5. Behavioral Limiter
- Tracks K/D, headshot %, accuracy
- Auto-reduces effectiveness if stats too high
- Forces deaths at emergency threshold

## Advanced Building

### Cross-Compilation
```powershell
# For 32-bit (if needed)
rustup target add i686-pc-windows-msvc
cargo build --release --target i686-pc-windows-msvc
```

### Debug Build
```powershell
# With debug symbols
cargo build

# Output: target\debug\rust-game-cheat.exe
```

### Custom Features
```toml
# In Cargo.toml, add features:
[features]
default = ["full"]
full = []
esp_only = []

# Build with specific feature:
cargo build --release --no-default-features --features esp_only
```

## File Structure

```
Rust-Game-EAC-Bypass-Cheat-v3.4---2026-1/
├── src/
│   ├── main.rs                    # Main entry point (v3.4 integrated)
│   ├── lib.rs                     # Library exports
│   ├── config.rs                  # Configuration system
│   ├── memory.rs                  # Memory operations
│   ├── scanner.rs                 # Pattern scanning
│   ├── offsets.rs                 # Game offsets
│   ├── eac_bypass.rs              # EAC bypass techniques
│   ├── driver_interface.rs        # Kernel driver interface
│   ├── esp_optimizer.rs           # ESP optimization (v3.2)
│   ├── recoil_helper.rs           # Recoil helper (v3.2)
│   ├── external_overlay.rs        # External overlay (v3.3)
│   ├── randomized_patterns.rs     # Randomization (v3.3)
│   ├── screenshot_detector.rs     # Screenshot protection (v3.3)
│   ├── behavioral_limiter.rs      # Behavioral limiter (v3.4)
│   ├── gradual_unlock.rs          # Gradual unlock (v3.4)
│   ├── anti_debug.rs              # Anti-debug (v3.4)
│   └── obfuscation.rs             # String obfuscation (v3.4)
├── Cargo.toml                     # Rust project config
├── build.rs                       # Build script (random names)
├── config.toml.example            # Config template
├── rename_build.ps1               # Windows build script
├── rename_build.sh                # Linux build script
├── README.md                      # Main documentation
├── UPGRADE_COMPLETE.md            # v3.4 upgrade notes
└── BUILD_GUIDE.md                 # This file
```

## Next Steps

1. ✅ Install Rust
2. ✅ Build with `rename_build.ps1`
3. ✅ Copy and edit `config.toml`
4. ✅ Run cheat
5. ⏳ Wait for gradual unlock (12 days)
6. ⏳ Monitor behavioral stats
7. ⏳ Update offsets when game updates

## Support

For issues:
1. Check this guide
2. Read UPGRADE_COMPLETE.md
3. Check README.md
4. Read COMMERCIAL_FEATURES.md

---

**v3.4 - Ultimate Safety Package**
**Detection risk: NEAR ZERO**
**Survival time: 12-24+ months**
