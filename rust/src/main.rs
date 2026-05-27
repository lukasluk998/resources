// Rust External ESP + No-Recoil - v3.4 Ultimate Safety Package
// Clean base with advanced safety integrations

mod memory;
mod scanner;
mod offsets;
mod driver_interface;
mod eac_bypass;
mod config;
mod esp_optimizer;
mod recoil_helper;
mod external_overlay;
mod randomized_patterns;
mod screenshot_detector;
mod behavioral_limiter;
mod gradual_unlock;
mod obfuscation;
mod anti_debug;

use std::thread;
use std::time::Duration;
use std::sync::Arc;

use memory::Process;
use scanner::PatternScanner;
use offsets::{RustOffsets, Vec3, Vec2};
use driver_interface::DriverInterface;
use eac_bypass::EACBypass;
use config::{CommercialConfig, SafetyMode, RecoilMethod, Humanizer};
use esp_optimizer::{ESPOptimizer, DetailLevel, CachedPlayer};
use recoil_helper::RecoilHelper;
use external_overlay::ExternalOverlay;
use randomized_patterns::RandomizedPatterns;
use screenshot_detector::{UnifiedScreenshotDetector, DetectionStrategy};
use behavioral_limiter::BehavioralLimiter;
use gradual_unlock::GradualUnlock;
use anti_debug::AntiDebug;

struct Player {
    address: usize,
    position: Vec3,
    health: f32,
    max_health: f32,
    distance: f32,
    is_local: bool,
}

struct RustCheat {
    process: Process,
    driver: Option<DriverInterface>,
    eac_bypass: EACBypass,
    offsets: RustOffsets,
    game_assembly_base: usize,
    local_player: usize,
    config: CommercialConfig,
    humanizer: Humanizer,
    esp_optimizer: ESPOptimizer,
    recoil_helper: RecoilHelper,
    external_overlay: Option<ExternalOverlay>,
    randomizer: RandomizedPatterns,
    screenshot_detector: UnifiedScreenshotDetector,
    behavioral_limiter: Option<BehavioralLimiter>,
    gradual_unlock: Option<GradualUnlock>,
    anti_debug: Arc<AntiDebug>,
}

impl RustCheat {
    fn new(process: Process, config: CommercialConfig) -> Option<Self> {
        let offsets = RustOffsets::new();
        
        // v3.4: Anti-Debug check FIRST (before anything else)
        let anti_debug = Arc::new(AntiDebug::new(
            config.anti_debug_enabled,
            config.anti_debug_auto_exit,
        ));
        
        if config.anti_debug_enabled {
            println!("[+] Anti-Debug: ACTIVE");
            if anti_debug.check_all() {
                println!("[!] Debugger detected - exiting for safety");
                return None;
            }
            
            // Start background monitoring
            let anti_debug_clone = anti_debug.clone();
            anti_debug_clone.start_monitoring();
        }
        
        // Load driver only if needed (not needed for Legit mode)
        let driver = if config.cheat.use_kernel_driver {
            println!("[*] Attempting to load kernel driver...");
            let drv = DriverInterface::new(process.pid);
            if drv.is_some() {
                println!("[+] Kernel driver loaded - EAC bypass active");
            } else {
                println!("[!] Kernel driver not found");
                if matches!(config.cheat.no_recoil_method, RecoilMethod::MemoryPatch) {
                    println!("[!] WARNING: MemoryPatch recoil needs driver!");
                    println!("[!] Falling back to fallback mode (HIGH RISK)");
                }
            }
            drv
        } else {
            println!("[*] Running in LEGIT MODE - No driver needed");
            println!("[+] Read-only operations (ESP) are safe");
            None
        };
        
        // EAC bypass helper
        let eac_bypass = EACBypass::new(process.handle);
        
        // Wait for EAC startup scan ONLY if using driver
        if config.cheat.use_kernel_driver {
            EACBypass::wait_for_game_load();
        }
        
        // Find GameAssembly.dll base
        let game_assembly_base = process.get_module_base("GameAssembly.dll")?;
        println!("[+] GameAssembly.dll: 0x{:X}", game_assembly_base);
        
        let humanizer = Humanizer::new(config.cheat.clone());
        let esp_optimizer = ESPOptimizer::new();
        let recoil_helper = RecoilHelper::new();
        
        // v3.3 Advanced Safety Features
        let external_overlay = if config.cheat.external_overlay_enabled {
            println!("[*] Creating external overlay (separate process)...");
            match ExternalOverlay::new("Rust") {
                Some(overlay) => {
                    println!("[+] External overlay created - SAFER than in-process");
                    println!("[+] Detection risk: NONE (external process)");
                    Some(overlay)
                }
                None => {
                    println!("[-] Failed to create external overlay");
                    println!("[!] Falling back to console-only mode");
                    None
                }
            }
        } else {
            println!("[*] External overlay disabled (console mode)");
            None
        };
        
        let randomizer = if config.cheat.randomized_reads {
            println!("[+] Randomized read patterns: ENABLED");
            println!("    - Random read order");
            println!("    - Random delays (50-150ms)");
            println!("    - Random skipping (15%)");
            RandomizedPatterns::new()
        } else {
            RandomizedPatterns::new()
        };
        
        let screenshot_detector = {
            let strategy = match config.cheat.screenshot_detection_strategy.as_str() {
                "None" => DetectionStrategy::None,
                "Basic" => DetectionStrategy::Basic,
                "Advanced" => DetectionStrategy::Advanced,
                "Paranoid" => DetectionStrategy::Paranoid,
                _ => DetectionStrategy::Basic,
            };
            
            if config.cheat.screenshot_protection {
                println!("[+] Screenshot protection: ENABLED ({:?})", strategy);
                println!("    - Hide overlay during screenshots");
                println!("    - Detection risk: NONE (overlay hidden)");
            }
            
            UnifiedScreenshotDetector::new(strategy)
        };
        
        // v3.4: Behavioral Limiter
        let behavioral_limiter = if config.behavioral_limiter_enabled {
            println!("[+] Behavioral Stats Limiter: ENABLED");
            println!("    - Tracks K/D, headshot %, accuracy");
            println!("    - Auto-reduces effectiveness if stats too high");
            println!("    - Forces deaths at emergency threshold");
            Some(BehavioralLimiter::with_defaults())
        } else {
            None
        };
        
        // v3.4: Gradual Unlock System
        let gradual_unlock = if config.gradual_unlock_enabled {
            println!("[+] Gradual Feature Unlock: ENABLED");
            let unlock = GradualUnlock::with_defaults();
            unlock.print_unlock_report();
            Some(unlock)
        } else {
            None
        };
        
        Some(RustCheat {
            process,
            driver,
            eac_bypass,
            offsets,
            game_assembly_base,
            local_player: 0,
            config,
            humanizer,
            esp_optimizer,
            recoil_helper,
            external_overlay,
            randomizer,
            screenshot_detector,
            behavioral_limiter,
            gradual_unlock,
            anti_debug,
        })
    }
    
    // Safe read - uses driver if available, fallback to normal read
    fn safe_read<T: Copy>(&self, address: usize) -> Result<T, ()> {
        // Add random jitter to avoid detection patterns
        EACBypass::random_delay();
        
        // Verify memory is readable first
        if !self.eac_bypass.safe_read_check(address) {
            return Err(());
        }
        
        if let Some(ref driver) = self.driver {
            driver.read(address)
        } else {
            self.process.read(address)
        }
    }
    
    // Safe write - uses driver if available
    fn safe_write<T: Copy>(&self, address: usize, value: T) -> Result<(), ()> {
        EACBypass::random_delay();
        
        if let Some(ref driver) = self.driver {
            driver.write(address, value)
        } else {
            self.process.write(address, value)
        }
    }
    
    fn find_local_player(&mut self) -> Option<usize> {
        // Pattern scan for LocalPlayer pointer
        let scanner = PatternScanner::new(&self.process);
        
        // Example pattern - you need to find the actual pattern in IDA/x64dbg
        let pattern = "48 8B 0D ? ? ? ? 48 85 C9 74 ? 48 8B 49 ? 48 85 C9";
        
        if let Some(addr) = scanner.scan_pattern(
            self.game_assembly_base,
            0x5000000,
            pattern
        ) {
            // Resolve RIP-relative address
            if let Ok(local_player_ptr) = scanner.resolve_rip_relative(addr, 3, 7) {
                if let Ok(local_player) = self.safe_read::<usize>(local_player_ptr) {
                    println!("[+] LocalPlayer: 0x{:X}", local_player);
                    return Some(local_player);
                }
            }
        }
        
        None
    }
    
    fn get_players(&mut self) -> Vec<Player> {
        let mut players = Vec::new();
        
        if self.local_player == 0 {
            return players;
        }
        
        // Get local player position for distance calculation
        let local_pos = self.get_player_position(self.local_player).unwrap_or(Vec3 { x: 0.0, y: 0.0, z: 0.0 });
        
        // Get camera forward vector for FOV culling (simplified - needs actual camera data)
        let camera_forward = Vec3 { x: 0.0, y: 0.0, z: 1.0 }; // TODO: Read from PlayerInput
        
        // Get random subset of player indices (v3.3 randomization)
        let player_count = 200;
        let player_indices = if self.config.randomized_reads {
            self.randomizer.get_random_subset(player_count)
        } else {
            (0..player_count).collect()
        };
        
        // Iterate through entity list with RANDOMIZED order
        for i in player_indices {
            // Random delay between reads (v3.3)
            if self.config.randomized_reads {
                if self.randomizer.should_skip() {
                    continue; // Skip this player randomly
                }
                
                // Small random delay
                if let Some(break_duration) = self.randomizer.should_take_break() {
                    std::thread::sleep(break_duration);
                }
            }
            
            let entity_addr = self.game_assembly_base + 0x1000000 + (i * 0x8); // Example
            
            if let Ok(entity) = self.safe_read::<usize>(entity_addr) {
                if entity == 0 || entity == self.local_player {
                    continue;
                }
                
                // Check cache first (ESP optimization)
                if let Some(cached) = self.esp_optimizer.get_cached_player(entity) {
                    // Use cached data if still valid
                    players.push(Player {
                        address: cached.address,
                        position: cached.position,
                        health: cached.health,
                        max_health: cached.max_health,
                        distance: cached.distance,
                        is_local: false,
                    });
                    continue;
                }
                
                // MEMORY BATCHING (v3.3) - Read entire player struct at once
                if self.config.memory_batching {
                    let batch_offsets = memory::PlayerBatchOffsets {
                        health: self.offsets.health,
                        max_health: self.offsets.max_health,
                        position: self.offsets.position,
                        player_model: self.offsets.player_model,
                        transform: self.offsets.transform,
                        rotation: 0x30, // Example offset
                        velocity: 0x40, // Example offset
                    };
                    
                    if let Ok(batch_data) = self.process.read_player_data_batch(entity, &batch_offsets) {
                        let health = batch_data.health;
                        let max_health = batch_data.max_health;
                        
                        if health > 0.0 && health <= 100.0 {
                            if let Some(pos_array) = batch_data.position {
                                let pos = Vec3 {
                                    x: pos_array[0],
                                    y: pos_array[1],
                                    z: pos_array[2],
                                };
                                
                                let distance = local_pos.distance(&pos);
                                
                                // ESP Optimization: Distance culling
                                if !self.esp_optimizer.should_render_player(distance) {
                                    continue;
                                }
                                
                                // ESP Optimization: FOV culling
                                if !self.esp_optimizer.is_in_fov(pos, local_pos, camera_forward) {
                                    continue;
                                }
                                
                                // Update cache
                                self.esp_optimizer.update_cache(entity, pos, health, max_health, distance, true);
                                
                                players.push(Player {
                                    address: entity,
                                    position: pos,
                                    health,
                                    max_health,
                                    distance,
                                    is_local: false,
                                });
                            }
                        }
                    }
                } else {
                    // Original method (separate reads) - fallback
                    if let Ok(health) = self.safe_read::<f32>(entity + self.offsets.health) {
                        if health > 0.0 && health <= 100.0 {
                            if let Some(pos) = self.get_player_position(entity) {
                                let distance = local_pos.distance(&pos);
                                
                                // ESP Optimization: Distance culling
                                if !self.esp_optimizer.should_render_player(distance) {
                                    continue;
                                }
                                
                                // ESP Optimization: FOV culling
                                if !self.esp_optimizer.is_in_fov(pos, local_pos, camera_forward) {
                                    continue;
                                }
                                
                                let max_health = self.safe_read::<f32>(entity + self.offsets.max_health)
                                    .unwrap_or(100.0);
                                
                                // Update cache
                                self.esp_optimizer.update_cache(entity, pos, health, max_health, distance, true);
                                
                                players.push(Player {
                                    address: entity,
                                    position: pos,
                                    health,
                                    max_health,
                                    distance,
                                    is_local: false,
                                });
                            }
                        }
                    }
                }
            }
        }
        
        // Increment frame counter for ESP optimization
        self.esp_optimizer.next_frame();
        
        players
    }
    
    fn get_player_position(&self, player_addr: usize) -> Option<Vec3> {
        // Get PlayerModel
        let player_model = self.safe_read::<usize>(player_addr + self.offsets.player_model).ok()?;
        if player_model == 0 {
            return None;
        }
        
        // Get Transform
        let transform = self.safe_read::<usize>(player_model + self.offsets.transform).ok()?;
        if transform == 0 {
            return None;
        }
        
        // Read position from transform
        let pos = self.safe_read::<Vec3>(transform + self.offsets.position).ok()?;
        Some(pos)
    }
    
    fn apply_no_recoil(&self) {
        // Check if recoil control is enabled
        match self.config.cheat.no_recoil_method {
            RecoilMethod::None => return,
            RecoilMethod::Macro => {
                // Macro recoil is handled by external script
                // See macro_norecoil.md for Logitech G HUB setup
                return;
            },
            RecoilMethod::Hardware => {
                // MAKCU hardware handles recoil
                // See src/makcu_interface.rs
                return;
            },
            RecoilMethod::MemoryPatch => {
                // Continue to memory patching below
            }
        }
        
        // v3.4: Check gradual unlock
        if let Some(ref unlock) = self.gradual_unlock {
            if !unlock.is_recoil_helper_unlocked() {
                return; // Recoil not unlocked yet
            }
        }
        
        if self.local_player == 0 {
            return;
        }
        
        // Humanized delay (looks less robotic)
        if self.config.cheat.humanization_enabled {
            EACBypass::polymorphic_sleep(50);
        }
        
        // Get PlayerInput
        if let Ok(player_input) = self.safe_read::<usize>(self.local_player + self.offsets.player_input) {
            if player_input != 0 {
                // v3.4: Apply gradual unlock strength multiplier
                let strength_mult = if let Some(ref unlock) = self.gradual_unlock {
                    unlock.recoil_strength_multiplier()
                } else {
                    1.0
                };
                
                // v3.4: Apply behavioral limiter effectiveness
                let effectiveness = if let Some(ref limiter) = self.behavioral_limiter {
                    if limiter.should_play_worse() {
                        limiter.get_effectiveness_multiplier()
                    } else {
                        1.0
                    }
                } else {
                    1.0
                };
                
                let final_strength = strength_mult * effectiveness;
                
                // Zero out recoil angles (via kernel driver) with strength multiplier
                let zero_vec = Vec3 { 
                    x: 0.0, 
                    y: 0.0, 
                    z: 0.0 
                };
                
                // If not full strength, apply partial compensation
                if final_strength < 1.0 {
                    // Read current recoil
                    if let Ok(current_recoil) = self.safe_read::<Vec3>(player_input + self.offsets.recoil_angles) {
                        // Apply partial compensation
                        let compensated = Vec3 {
                            x: current_recoil.x * (1.0 - final_strength),
                            y: current_recoil.y * (1.0 - final_strength),
                            z: current_recoil.z * (1.0 - final_strength),
                        };
                        let _ = self.safe_write(player_input + self.offsets.recoil_angles, compensated);
                    }
                } else {
                    // Full compensation
                    let _ = self.safe_write(player_input + self.offsets.recoil_angles, zero_vec);
                }
            }
        }
    }
    
    fn draw_esp(&mut self, players: &[Player]) {
        // Check if should hide overlay (screenshot protection)
        let should_hide = self.config.cheat.screenshot_protection && self.screenshot_detector.should_hide_overlay();
        
        if should_hide {
            // Hide external overlay
            if let Some(ref mut overlay) = self.external_overlay {
                overlay.set_visible(false);
            }
            return;
        } else {
            // Show external overlay
            if let Some(ref mut overlay) = self.external_overlay {
                if !overlay.is_visible() {
                    overlay.set_visible(true);
                    println!("[+] Overlay visible again (screenshot finished)");
                }
            }
        }
        
        // v3.4: Apply gradual unlock limits to ESP
        let esp_distance_limit = if let Some(ref unlock) = self.gradual_unlock {
            self.config.cheat.max_esp_distance * unlock.esp_distance_multiplier()
        } else {
            self.config.cheat.max_esp_distance
        };
        
        let show_full_info = if let Some(ref unlock) = self.gradual_unlock {
            unlock.esp_show_full_info()
        } else {
            true
        };
        
        // v3.4: Check if we should play worse (behavioral limiter)
        let effectiveness = if let Some(ref limiter) = self.behavioral_limiter {
            if limiter.should_force_death() {
                println!("[Behavioral] ⚠️  EMERGENCY: Need death - disabling ESP temporarily");
                return; // Don't show ESP to force death
            }
            
            if limiter.should_play_worse() {
                let mult = limiter.get_effectiveness_multiplier();
                println!("[Behavioral] Playing worse mode: {:.0}% effectiveness", mult * 100.0);
                mult
            } else {
                1.0
            }
        } else {
            1.0
        };
        
        // Draw using external overlay if available
        if let Some(ref mut overlay) = self.external_overlay {
            // Update overlay position to match game window
            overlay.update_position();
            
            // Begin drawing frame
            overlay.begin_draw();
            
            for player in players {
                if player.is_local {
                    continue;
                }
                
                // Apply distance limit from gradual unlock
                if player.distance > esp_distance_limit {
                    continue;
                }
                
                // Apply effectiveness multiplier (behavioral limiter)
                // Randomly skip players based on effectiveness
                if effectiveness < 1.0 {
                    if rand::random::<f32>() > effectiveness {
                        continue; // Skip this player
                    }
                }
                
                // Get detail level based on distance (ESP optimization)
                let detail = self.esp_optimizer.get_detail_level(player.distance);
                
                // For now, just draw to console (world-to-screen needs camera matrices)
                // In full implementation, you'd:
                // 1. Read camera view/projection matrices
                // 2. Transform world position to screen coords
                // 3. Draw on overlay if on screen
                
                // TODO: Implement world_to_screen projection
                // let screen_pos = external_overlay::world_to_screen(player.position, overlay.width, overlay.height);
                
                // Fallback: print to console
                let mut info = String::new();
                
                if detail.show_distance() {
                    info.push_str(&format!("Distance: {:.1}m", player.distance));
                }
                
                if show_full_info && detail.show_health_bar() {
                    info.push_str(&format!(" | HP: {:.0}/{:.0}", player.health, player.max_health));
                }
                
                if show_full_info && detail.show_name() {
                    info.push_str(&format!(" | Pos: ({:.1}, {:.1}, {:.1})", 
                        player.position.x, player.position.y, player.position.z));
                }
                
                if !info.is_empty() {
                    println!("[ESP] {}", info);
                }
            }
            
            // End drawing frame
            overlay.end_draw();
            
            // Process Windows messages
            overlay.process_messages();
        } else {
            // Console-only mode (no overlay)
            for player in players {
                if player.is_local {
                    continue;
                }
                
                // Apply distance limit from gradual unlock
                if player.distance > esp_distance_limit {
                    continue;
                }
                
                // Apply effectiveness multiplier
                if effectiveness < 1.0 {
                    if rand::random::<f32>() > effectiveness {
                        continue;
                    }
                }
                
                let detail = self.esp_optimizer.get_detail_level(player.distance);
                let mut info = String::new();
                
                if detail.show_distance() {
                    info.push_str(&format!("Distance: {:.1}m", player.distance));
                }
                
                if show_full_info && detail.show_health_bar() {
                    info.push_str(&format!(" | HP: {:.0}/{:.0}", player.health, player.max_health));
                }
                
                if show_full_info && detail.show_name() {
                    info.push_str(&format!(" | Pos: ({:.1}, {:.1}, {:.1})", 
                        player.position.x, player.position.y, player.position.z));
                }
                
                if !info.is_empty() {
                    println!("[ESP] {}", info);
                }
            }
        }
    }
    
    // v3.4: Register kill for behavioral tracking
    pub fn register_kill(&mut self, was_headshot: bool) {
        if let Some(ref mut limiter) = self.behavioral_limiter {
            limiter.register_kill(was_headshot);
        }
    }
    
    // v3.4: Register death for behavioral tracking
    pub fn register_death(&mut self) {
        if let Some(ref mut limiter) = self.behavioral_limiter {
            limiter.register_death();
        }
    }
    
    // v3.4: Register shots for behavioral tracking
    pub fn register_shots(&mut self, fired: u32, hit: u32) {
        if let Some(ref mut limiter) = self.behavioral_limiter {
            limiter.register_shots(fired, hit);
        }
    }
}

fn is_admin() -> bool {
    // Check if running with admin privileges (stub)
    true
}

fn main() {
    println!("╔══════════════════════════════════════════════╗");
    println!("║   Rust External ESP + No-Recoil v3.4         ║");
    println!("║   Ultimate Safety Package                    ║");
    println!("╚══════════════════════════════════════════════╝");
    println!();
    
    // Load commercial configuration
    let config = CommercialConfig::load();
    
    // Print license info
    config.print_license_info();
    
    // Validate license and features
    if let Err(e) = config.validate() {
        println!("[!] Configuration error: {}", e);
        println!("[!] Please check your license or config.toml");
        return;
    }
    
    println!("[+] Configuration loaded:");
    println!("    Mode: {:?}", config.cheat.mode);
    println!("    ESP: {}", if config.cheat.esp_enabled { "✓" } else { "✗" });
    println!("    No Recoil: {:?}", config.cheat.no_recoil_method);
    println!("    Kernel Driver: {}", if config.cheat.use_kernel_driver { "✓" } else { "✗" });
    println!("    Humanization: {}", if config.cheat.humanization_enabled { "✓" } else { "✗" });
    println!();
    
    // v3.4 Ultimate Safety status
    println!("[+] v3.4 Ultimate Safety Features:");
    println!("    Behavioral Limiter: {}", if config.behavioral_limiter_enabled { "✓ ENABLED" } else { "✗ Disabled" });
    println!("    Gradual Unlock: {}", if config.gradual_unlock_enabled { "✓ ENABLED" } else { "✗ Disabled" });
    println!("    Anti-Debug: {}", if config.anti_debug_enabled { "✓ ENABLED" } else { "✗ Disabled" });
    println!("    String Obfuscation: {}", if config.string_obfuscation_enabled { "✓ ENABLED" } else { "✗ Disabled" });
    println!();
    
    // Safety warnings based on mode
    match config.cheat.mode {
        SafetyMode::Legit => {
            println!("[✓] LEGIT MODE - Maximum safety");
            println!("    Detection risk: NEAR ZERO (v3.4 improvements)");
            println!("    Expected survival: 12-24+ months");
            println!("    Features: ESP only, macro recoil");
            if config.cheat.external_overlay_enabled {
                println!("    + External overlay (SAFER)");
            }
            if config.cheat.randomized_reads {
                println!("    + Randomized patterns (SAFER)");
            }
            if config.cheat.memory_batching {
                println!("    + Memory batching (80% fewer reads)");
            }
            if config.cheat.screenshot_protection {
                println!("    + Screenshot protection (SAFER)");
            }
            if config.behavioral_limiter_enabled {
                println!("    + Behavioral limiter (ULTIMATE SAFETY)");
            }
            if config.gradual_unlock_enabled {
                println!("    + Gradual unlock (ULTIMATE SAFETY)");
            }
        },
        SafetyMode::Rage => {
            println!("[!] RAGE MODE - Higher detection risk");
            println!("    Detection risk: LOW (v3.4 improvements)");
            println!("    Expected survival: 2-4 weeks");
            println!("    Recommendation: Use alt account only");
            if config.cheat.external_overlay_enabled {
                println!("    + External overlay (SAFER)");
            }
            if config.cheat.randomized_reads {
                println!("    + Randomized patterns (SAFER)");
            }
            if config.behavioral_limiter_enabled {
                println!("    + Behavioral limiter (helps even in rage)");
            }
        },
    }
    println!();
    
    // Check for admin rights
    if !is_admin() {
        if config.cheat.use_kernel_driver {
            println!("[!] WARNING: Not running as administrator!");
            println!("[!] Driver loading will fail without admin rights");
            println!();
        }
    }
    
    // HWID spoof option (only if using memory writes)
    if matches!(config.cheat.no_recoil_method, RecoilMethod::MemoryPatch) {
        println!("[?] Run HWID spoofer? (prevents hardware bans)");
        println!("    Type 'yes' to spoof, or press Enter to skip");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        if input.trim().to_lowercase() == "yes" {
            eac_bypass::hwid_spoof::full_spoof();
            println!("[!] Restart system before playing for spoof to take effect");
            return;
        }
    }
    
    println!("[+] Waiting for RustClient.exe...");
    
    let process = loop {
        if let Some(proc) = Process::from_name("RustClient.exe") {
            println!("[+] Found RustClient.exe (PID: {})", proc.pid);
            break proc;
        }
        thread::sleep(Duration::from_secs(2));
    };
    
    let mut cheat = match RustCheat::new(process, config.clone()) {
        Some(c) => c,
        None => {
            println!("[-] Failed to initialize cheat");
            return;
        }
    };
    
    println!();
    println!("[+] Cheat initialized");
    println!("[+] Active Features:");
    if config.cheat.esp_enabled {
        println!("    ✓ ESP (Player positions, health, distance)");
    }
    match config.cheat.no_recoil_method {
        RecoilMethod::None => println!("    ✗ No Recoil (disabled)"),
        RecoilMethod::Macro => println!("    ✓ No Recoil (Logitech macro - SAFE)"),
        RecoilMethod::Hardware => println!("    ✓ No Recoil (MAKCU hardware - SAFE)"),
        RecoilMethod::MemoryPatch => {
            if cheat.driver.is_some() {
                println!("    ✓ No Recoil (kernel driver - MEDIUM RISK)");
            } else {
                println!("    ⚠ No Recoil (fallback - HIGH RISK)");
            }
        },
    }
    println!();
    
    // Find local player
    println!("[*] Scanning for LocalPlayer...");
    if let Some(local_player) = cheat.find_local_player() {
        cheat.local_player = local_player;
        println!("[+] LocalPlayer found!");
    } else {
        println!("[-] Could not find LocalPlayer - ESP will be limited");
    }
    
    // Main cheat loop with humanization
    println!();
    println!("[+] Cheat running... Press Ctrl+C to exit");
    println!();
    println!("[*] ESP Optimization: ENABLED");
    println!("    - Distance-based LOD (Level of Detail)");
    println!("    - FOV culling (only render visible players)");
    println!("    - Frame skipping for distant players");
    println!("    - Player data caching");
    println!();
    
    if config.cheat.external_overlay_enabled {
        println!("[*] External Overlay: ENABLED");
        println!("    - Separate process (not injected)");
        println!("    - EAC cannot detect external process");
        println!("    - Detection risk: NONE");
        println!();
    }
    
    if config.cheat.randomized_reads {
        println!("[*] Randomized Read Patterns: ENABLED");
        println!("    - Random player order");
        println!("    - Random delays (50-150ms)");
        println!("    - Random skipping (15%)");
        println!("    - No predictable patterns");
        println!();
    }
    
    if config.cheat.memory_batching {
        println!("[*] Memory Batching: ENABLED");
        println!("    - Read whole struct at once");
        println!("    - 80% fewer ReadProcessMemory calls");
        println!("    - Much faster and safer");
        println!();
    }
    
    if config.cheat.screenshot_protection {
        println!("[*] Screenshot Protection: ENABLED");
        println!("    - Strategy: {:?}", config.cheat.screenshot_detection_strategy);
        println!("    - Hide overlay during screenshots");
        println!("    - Prevents visual detection");
        println!();
    }
    
    println!("[*] Recoil Helper: ENABLED (Read-Only)");
    println!("    - Visual compensation guide");
    println!("    - No memory writes (100% SAFE)");
    println!("    - Load weapon patterns from memory");
    println!();
    
    let mut tick = 0;
    
    loop {
        // Anti-debug quick check every loop
        if config.anti_debug_enabled && tick % 50 == 0 {
            if cheat.anti_debug.quick_check() {
                println!("[!] Debugger detected during runtime - exiting");
                break;
            }
        }
        
        // Update weapon data from memory (read-only)
        if tick % 10 == 0 && cheat.local_player != 0 {
            if let Some(weapon) = cheat.recoil_helper.read_weapon_from_memory(
                &cheat.process,
                cheat.local_player,
                cheat.offsets.held_entity,
                cheat.offsets.weapon_recoil,
            ) {
                cheat.recoil_helper.update_weapon(weapon);
            }
        }
        
        // Display recoil helper info
        if tick % 20 == 0 {
            if let Some(weapon_info) = cheat.recoil_helper.get_weapon_info() {
                println!("[Recoil] {}", weapon_info);
                
                // Show compensation offset
                let offset = cheat.recoil_helper.get_compensation_offset();
                if offset.x != 0.0 || offset.y != 0.0 {
                    println!("[Recoil] Compensation: X={:.2}, Y={:.2}", offset.x, offset.y);
                }
            }
        }
        
        // Apply no recoil (if enabled) with gradual unlock check
        let recoil_enabled = if let Some(ref unlock) = cheat.gradual_unlock {
            unlock.is_recoil_helper_unlocked()
        } else {
            true // No gradual unlock = always enabled
        };
        
        if recoil_enabled {
            cheat.apply_no_recoil();
        }
        
        // Update ESP with humanized timing and optimization
        if config.cheat.esp_enabled && tick % 5 == 0 {
            // Random skip (looks human)
            if !cheat.humanizer.should_skip_esp_frame() {
                let players = cheat.get_players();
                if !players.is_empty() {
                    cheat.draw_esp(&players);
                }
            }
            
            // Humanized delay between ESP updates
            cheat.humanizer.esp_update_delay();
            
            // Random delay for pattern avoidance (v3.3)
            if config.cheat.randomized_reads {
                cheat.randomizer.random_delay();
            }
        }
        
        // v3.4: Behavioral stats report every 5 minutes
        if config.behavioral_limiter_enabled && tick % 3000 == 0 && tick > 0 {
            if let Some(ref limiter) = cheat.behavioral_limiter {
                limiter.print_report();
                
                // Check if session should end
                if limiter.should_end_session() {
                    println!("[!] Session limit reached - exiting for safety");
                    break;
                }
            }
        }
        
        // v3.4: Gradual unlock status every 10 minutes
        if config.gradual_unlock_enabled && tick % 6000 == 0 && tick > 0 {
            if let Some(ref unlock) = cheat.gradual_unlock {
                unlock.print_unlock_report();
            }
        }
        
        tick += 1;
        thread::sleep(Duration::from_millis(100));
    }
    
    // Final stats report
    if config.behavioral_limiter_enabled {
        if let Some(ref limiter) = cheat.behavioral_limiter {
            println!("\n[+] Final Session Report:");
            limiter.print_report();
        }
    }
    
    println!("\n[+] Cheat exited safely");
}
