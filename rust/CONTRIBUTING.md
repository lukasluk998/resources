# Contributing to Rust External

Thank you for your interest in contributing! This project is for educational purposes only.

## Code of Conduct

- Be respectful to all contributors
- No harassment or offensive language
- Focus on technical discussions
- Remember this is for learning, not for ruining games

## How to Contribute

### Reporting Issues

1. Check if the issue already exists
2. Provide detailed information:
   - Operating system and version
   - Rust version (`rustc --version`)
   - Game version
   - Steps to reproduce
   - Error messages or logs

3. **IMPORTANT:** Only report issues tested on alt accounts

### Submitting Changes

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Make your changes
4. Test thoroughly on alt account
5. Commit with clear messages: `git commit -m "Add feature: description"`
6. Push to your fork: `git push origin feature/your-feature`
7. Open a Pull Request

### Pull Request Guidelines

- Describe what your PR does
- Reference any related issues
- Include testing results (on alt account)
- Follow existing code style
- Update documentation if needed

## Development Setup

### Prerequisites

```powershell
# Install Rust
https://rustup.rs/

# Install WDK (for driver development)
https://docs.microsoft.com/en-us/windows-hardware/drivers/download-the-wdk

# Install Git
https://git-scm.com/
```

### Building

```powershell
# Clone repo
git clone https://github.com/lukasluk998/Rust-Game-EAC-Bypass-Cheat-v3.4---2026.git
cd Rust-Game-EAC-Bypass-Cheat-v3.4---2026

# Build cheat
cargo build --release

# Build driver (requires WDK)
cd driver
msbuild driver.vcxproj /p:Configuration=Release /p:Platform=x64
```

### Testing

**CRITICAL:** Only test on throwaway accounts!

1. Create a new Steam account
2. Buy Rust on sale (or use family sharing)
3. Test your changes
4. Document results
5. Never test on main account

## What to Contribute

### Welcome Contributions:

✅ Bug fixes
✅ Performance improvements
✅ Documentation improvements
✅ Code cleanup
✅ New detection avoidance techniques
✅ Offset updates (when game updates)
✅ Pattern improvements

### Not Welcome:

❌ Malicious code
❌ Backdoors or trojans
❌ Obvious detection vectors
❌ Paid features or paywalls
❌ Anything that harms the community

## Code Style

### Rust Code:

```rust
// Use descriptive names
fn get_player_position(&self, player_addr: usize) -> Option<Vec3> {
    // Clear comments
    let player_model = self.read::<usize>(player_addr + self.offsets.player_model).ok()?;
    
    // Proper error handling
    if player_model == 0 {
        return None;
    }
    
    // Return result
    Some(position)
}
```

### C Code (Driver):

```c
// Clear function names
NTSTATUS ReadProcessMemory(PEPROCESS Process, PVOID Address, PVOID Buffer, SIZE_T Size) {
    SIZE_T bytesRead;
    
    // Use MmCopyVirtualMemory (not ZwReadVirtualMemory)
    return MmCopyVirtualMemory(
        Process,
        Address,
        PsGetCurrentProcess(),
        Buffer,
        Size,
        KernelMode,
        &bytesRead
    );
}
```

## Documentation

When adding features:

1. Update README.md
2. Add comments in code
3. Update relevant docs in `docs/`
4. Include usage examples

## Security

### Reporting Vulnerabilities:

If you find a security issue:
1. **DO NOT** open a public issue
2. Email the maintainers privately
3. Wait for response before disclosure

### Driver Security:

- Never include backdoors
- No telemetry or data collection
- No network connections
- Keep driver code clean and auditable

## Offset Updates

When game updates:

1. Find new patterns using IDA/Ghidra
2. Update `src/scanner.rs`
3. Test on alt account
4. Submit PR with:
   - Game version
   - New patterns
   - Test results

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Questions?

- Open an issue for questions
- Check existing issues first
- Be patient, maintainers are volunteers

## Remember

This project is for **educational purposes only**. We do not condone cheating in online games. Use responsibly and only on throwaway accounts.

---

Thank you for contributing! 🙏
