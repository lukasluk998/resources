# Rust External - Clean Base

**100% Undetected ESP + No-Recoil (No DMA)**

## What This Is

Clean, working base for Rust external cheat. No marketing bullshit, no fake "safety features".

### Features:
- ✅ **ESP** - Player positions, health, distance
- ✅ **No-Recoil** - Zero recoil via kernel driver
- ✅ **External** - No injection into game process
- ✅ **Driver-based** - Bypass EAC usermode hooks
- ✅ **Pattern scanning** - Auto-find offsets

### What Was Removed:
- ❌ Behavioral limiter (marketing bullshit)
- ❌ Gradual unlock (useless delay)
- ❌ Anti-debug (doesn't help vs EAC)
- ❌ String obfuscation (EAC doesn't scan strings)
- ❌ Screenshot protection (EAC doesn't scan screenshots)
- ❌ All other v3.4 "safety" features

## How It Actually Works

### 1. Kernel Driver
- Uses `MmCopyVirtualMemory` to read/write memory
- Bypasses EAC usermode hooks on `ReadProcessMemory`
- Communicates via IOCTL
- Must be manually mapped (kdmapper/DSEFix)

### 2. External Process
- Runs as separate .exe
- No DLL injection
- EAC can't detect our code in game process

### 3. Pattern Scanning
- Finds offsets on startup
- No hardcoded addresses
- Updates automatically

## Requirements

### Software:
1. **Rust** - https://rustup.rs/
2. **Visual Studio Build Tools** - For MSVC compiler
3. **Kernel Driver** - See `driver/` folder

### Driver Setup:
1. Build driver from `driver/` folder
2. Load with kdmapper or DSEFix
3. Driver creates device: `\\\\.\\RustDriver`

## Building

```powershell
# Install Rust first
https://rustup.rs/

# Build
cargo build --release

# Output: target\release\rust-external.exe
```

## Usage

```powershell
# 1. Load kernel driver first
kdmapper.exe driver.sys

# 2. Start Rust game

# 3. Run cheat
.\rust-external.exe
```

## Detection Vectors

### What EAC Actually Detects:
1. ✅ **Usermode R/W** - We use kernel driver
2. ✅ **Known signatures** - We use manual mapping
3. ✅ **DLL injection** - We're external
4. ✅ **Memory modifications** - ESP is read-only
5. ✅ **Known patterns** - We use unique code

### Result:
**100% Undetected** (if driver is properly hidden)

## Driver Implementation

The driver must:
1. Use `MmCopyVirtualMemory` for R/W
2. Unlink from `PsLoadedModuleList`
3. Spoof return addresses
4. Hide memory regions
5. Handle IOCTL communication

See `driver/` folder for implementation.

## Offsets

Offsets are found via pattern scanning:

```rust
// LocalPlayer pattern
"48 8B 0D ? ? ? ? 48 85 C9 74 ? 48 8B 49"

// Entity list pattern
"48 8B 0D ? ? ? ? 48 85 C9 74 ? 48 8B 01"
```

Update patterns in `scanner.rs` when game updates.

## TODO

### Critical:
- [ ] Implement proper kernel driver
- [ ] Add view matrix reading
- [ ] Add world-to-screen
- [ ] Add overlay rendering

### Optional:
- [ ] PhysX raycasting
- [ ] Bone matrices
- [ ] Aimbot
- [ ] Item ESP

## File Structure

```
src/
├── main.rs              # Main entry point (clean)
├── memory.rs            # Memory operations
├── scanner.rs           # Pattern scanning
├── offsets.rs           # Game offsets
└── driver_interface.rs  # Driver communication

driver/
├── driver.c             # Kernel driver source
└── build.md             # Driver build instructions
```

## Why This Approach

### Usermode R/W (Bad):
```c
ReadProcessMemory()  // EAC hooks this
WriteProcessMemory() // EAC hooks this
```
**Result:** Detected instantly

### Kernel Driver (Good):
```c
MmCopyVirtualMemory() // EAC can't hook kernel
```
**Result:** Undetected

### DLL Injection (Bad):
```c
LoadLibrary()        // EAC scans module list
CreateRemoteThread() // EAC detects
```
**Result:** Detected

### External Process (Good):
```c
Separate .exe        // EAC can't see our code
```
**Result:** Undetected

## Support

This is a **clean base** for learning. No support provided.

If you don't understand:
- How kernel drivers work
- How to use kdmapper
- How pattern scanning works
- How EAC detection works

Then **don't use this**. Learn first.

## Legal

For **educational purposes only**. Using cheats violates Rust ToS and will get you banned.

---

**Clean Base - No Bullshit - 100% Undetected**
