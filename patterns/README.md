# Patterns Directory

Weapon recoil patterns, AOB signatures, and entity structures.

## Weapon Patterns (`weapons/`)

JSON format for each weapon:

```json
{
  "weapon": "AK47",
  "fire_rate": 8.33,
  "pattern": [
    [0, 2], [0, 3], [0, 4], [0, 5],
    [1, 6], [2, 6], [3, 5], [4, 4]
  ],
  "standing_mult": 1.0,
  "crouching_mult": 0.85,
  "notes": "Spray up-right, then compensate left"
}
```

Pattern format: `[x_offset, y_offset]` per shot

## AOB Signatures (`aob/`)

Memory patterns for critical pointers:

```
LocalPlayer: 48 8B 0D ? ? ? ? 48 85 C9 74 ? 48 8B 49
LocalPlayer_Alt: 48 8B 0D ? ? ? ? 48 85 C9 0F 84
ClientEntities: 48 8B 05 ? ? ? ? 48 8B 88 ? ? ? ? 48 85 C9
GameObjectManager: 48 8B 0D ? ? ? ? 48 8B 01 48 8B 40
```

Format: Hex bytes with `?` wildcards

## Entity Structures (`entities/`)

ReClass.NET exports or manual docs for:
- BasePlayer structure
- BaseNetworkable
- PlayerModel
- PlayerInput
- Weapon/Item structures

## Usage

These patterns are used by the runtime dumper to find offsets automatically.

### Adding New Patterns

1. Find with Cheat Engine or x64dbg
2. Make generic with wildcards
3. Test on multiple game versions
4. Add to appropriate subfolder
5. Commit with description

### Pattern Format Rules

- Use uppercase hex: `48 8B 0D`
- Wildcards: `?` or `??`
- Space-separated
- Comments with `//` on same line
- Include RIP-relative info if applicable

Example:
```
// LocalPlayer singleton pattern (RIP+3, instruction len 7)
48 8B 0D ? ? ? ? 48 85 C9 74
```
