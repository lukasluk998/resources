// Rust game offsets - BUILD 23369401 (2026-05-25)
// Updated from UnknownCheats forum - FRESH offsets
// Includes encrypted value decryptors

use std::ptr;

pub struct RustOffsets {
    // GameAssembly.dll base offsets
    pub timestamp: usize,
    pub il2cpp_domain_get: usize,
    pub il2cpp_resolve_icall: usize,
    
    // BaseNetworkable chain (ENCRYPTED)
    pub base_networkable_static: usize,       // 0xe290810
    pub client_entities_offset: usize,         // 0x18 (from static)
    pub entity_realm_offset: usize,            // 0x10
    pub entity_list_vals: usize,               // 0x18 (ListDictionary)
    
    // BasePlayer offsets
    pub base_player_typeinfo: usize,           // 0xe27c4a8
    pub player_model: usize,                   // 0x408
    pub input: usize,                          // 0x628
    pub movement: usize,                       // 0x488
    pub cl_active_item: usize,                 // 0x510 (ENCRYPTED)
    pub model_state: usize,                    // 0x380
    pub player_flags: usize,                   // 0x658
    pub eyes: usize,                           // 0x3c8
    pub user_id: usize,                        // 0x6a0
    pub inventory: usize,                      // 0x690
    pub display_name: usize,                   // 0x560
    pub view_matrix: usize,                    // 0x30c
    pub mounted: usize,                        // 0x568
    pub weapon_move_speed_scale: usize,        // 0x738
    
    // BaseCombatEntity offsets
    pub skeleton_properties: usize,            // 0x208
    pub base_protection: usize,                // 0x210
    pub lifestate: usize,                      // 0x280
    pub health: usize,                         // 0x28c
    pub max_health: usize,                     // 0x290
    
    // BaseEntity offsets
    pub bounds: usize,                         // 0x16c
    pub model: usize,                          // 0x198
    pub flags: usize,                          // 0x1a0
    pub position_lerp: usize,                  // 0xc8
    
    // PlayerModel offsets
    pub multi_mesh: usize,                     // 0xa0
    pub player_model_position: usize,          // 0x2b8
    pub new_velocity: usize,                   // 0x2dc
    
    // Transform offsets
    pub get_position_injected: usize,          // 0xcfcc0 (RVA from GameAssembly)
    pub get_rotation_injected: usize,          // 0xcfde0
    
    // Model offsets
    pub root_bone: usize,                      // 0x28
    pub head_bone: usize,                      // 0x30
    pub eye_bone: usize,                       // 0x38
    pub bone_transforms: usize,                // 0x50
    
    // PlayerInput offsets
    pub input_state: usize,                    // 0x28
    pub body_angles: usize,                    // 0x44
    
    // InputState offsets
    pub current: usize,                        // 0x10
    pub previous: usize,                       // 0x18
    
    // ModelState offsets
    pub water_level: usize,                    // 0x5c
    pub look_dir: usize,                       // 0x68
    
    // BaseProjectile offsets
    pub base_projectile_typeinfo: usize,      // 0xe27c300
    pub projectile_velocity_scale: usize,     // 0x334
    pub automatic: usize,                      // 0x338
    pub reload_time: usize,                    // 0x378
    pub primary_magazine: usize,               // 0x380
    pub aim_sway: usize,                       // 0x3a0
    pub aim_sway_speed: usize,                 // 0x3a4
    pub recoil: usize,                         // 0x3a8
    pub aimcone_curve: usize,                  // 0x3b0
    pub aim_cone: usize,                       // 0x3b8
    pub hip_aim_cone: usize,                   // 0x3bc
    pub sight_aimcone_scale: usize,            // 0x414
    pub hip_aimcone_scale: usize,              // 0x41c
    
    // RecoilProperties offsets
    pub recoil_yaw_min: usize,                 // 0x18
    pub recoil_yaw_max: usize,                 // 0x1c
    pub recoil_pitch_min: usize,               // 0x20
    pub recoil_pitch_max: usize,               // 0x24
    
    // Magazine offsets
    pub magazine_capacity: usize,              // 0x18
    pub magazine_contents: usize,              // 0x1c
    pub ammo_type: usize,                      // 0x20
    
    // MainCamera offsets
    pub main_camera_static: usize,             // 0xe275c98
    pub main_camera_instance: usize,           // 0x20 (static field)
    pub main_camera: usize,                    // 0x48
    pub main_camera_transform: usize,          // 0x60
    
    // Camera functions
    pub camera_world_to_screen: usize,         // 0x7a230 (RVA)
}

impl RustOffsets {
    pub fn new() -> Self {
        Self {
            // GameAssembly base
            timestamp: 0x6a109e98,
            il2cpp_domain_get: 0x81a110,
            il2cpp_resolve_icall: 0x8193d0,
            
            // BaseNetworkable (encrypted chain)
            base_networkable_static: 0xe290810,
            client_entities_offset: 0x18,
            entity_realm_offset: 0x10,
            entity_list_vals: 0x18,
            
            // BasePlayer
            base_player_typeinfo: 0xe27c4a8,
            player_model: 0x408,
            input: 0x628,
            movement: 0x488,
            cl_active_item: 0x510,  // ENCRYPTED - needs decrypt_cl_active_item()
            model_state: 0x380,
            player_flags: 0x658,
            eyes: 0x3c8,
            user_id: 0x6a0,
            inventory: 0x690,
            display_name: 0x560,
            view_matrix: 0x30c,
            mounted: 0x568,
            weapon_move_speed_scale: 0x738,
            
            // BaseCombatEntity
            skeleton_properties: 0x208,
            base_protection: 0x210,
            lifestate: 0x280,
            health: 0x28c,
            max_health: 0x290,
            
            // BaseEntity
            bounds: 0x16c,
            model: 0x198,
            flags: 0x1a0,
            position_lerp: 0xc8,
            
            // PlayerModel
            multi_mesh: 0xa0,
            player_model_position: 0x2b8,
            new_velocity: 0x2dc,
            
            // Transform functions (RVA from GameAssembly)
            get_position_injected: 0xcfcc0,
            get_rotation_injected: 0xcfde0,
            
            // Model
            root_bone: 0x28,
            head_bone: 0x30,
            eye_bone: 0x38,
            bone_transforms: 0x50,
            
            // PlayerInput
            input_state: 0x28,
            body_angles: 0x44,
            
            // InputState
            current: 0x10,
            previous: 0x18,
            
            // ModelState
            water_level: 0x5c,
            look_dir: 0x68,
            
            // BaseProjectile
            base_projectile_typeinfo: 0xe27c300,
            projectile_velocity_scale: 0x334,
            automatic: 0x338,
            reload_time: 0x378,
            primary_magazine: 0x380,
            aim_sway: 0x3a0,
            aim_sway_speed: 0x3a4,
            recoil: 0x3a8,
            aimcone_curve: 0x3b0,
            aim_cone: 0x3b8,
            hip_aim_cone: 0x3bc,
            sight_aimcone_scale: 0x414,
            hip_aimcone_scale: 0x41c,
            
            // RecoilProperties
            recoil_yaw_min: 0x18,
            recoil_yaw_max: 0x1c,
            recoil_pitch_min: 0x20,
            recoil_pitch_max: 0x24,
            
            // Magazine
            magazine_capacity: 0x18,
            magazine_contents: 0x1c,
            ammo_type: 0x20,
            
            // MainCamera
            main_camera_static: 0xe275c98,
            main_camera_instance: 0x20,
            main_camera: 0x48,
            main_camera_transform: 0x60,
            
            // Camera functions
            camera_world_to_screen: 0x7a230,
        }
    }
}

// ============================================================================
// ENCRYPTED VALUE DECRYPTORS - BUILD 23369401
// ============================================================================

/// Decrypt cl_active_item (BasePlayer->clActiveItem encrypted pointer)
/// Auto-generated from UnknownCheats
pub fn decrypt_cl_active_item(encrypted: u64) -> u64 {
    let mut value = encrypted;
    let chunks = unsafe { std::slice::from_raw_parts_mut(&mut value as *mut u64 as *mut u32, 2) };
    
    for i in 0..2 {
        let mut v = chunks[i];
        v = v.wrapping_add(0xEF263B74);
        v = v.rotate_right(25);  // ROR 25
        v = v.wrapping_add(0x6426B5F9);
        v = v.rotate_right(7);   // ROR 7
        chunks[i] = v;
    }
    
    value
}

/// Decrypt client_entities (BaseNetworkable.Static->clientEntities)
/// Returns Il2Cpp handle that needs Il2cppGetHandle() call
pub fn decrypt_client_entities(encrypted: u64) -> u64 {
    let mut value = encrypted;
    let chunks = unsafe { std::slice::from_raw_parts_mut(&mut value as *mut u64 as *mut u32, 2) };
    
    for i in 0..2 {
        let mut v = chunks[i];
        v = v.wrapping_add(0x4BA08177);
        v = v.rotate_left(24);  // ROL 24
        v = v.wrapping_add(0xa20de23c);
        chunks[i] = v;
    }
    
    value
}

/// Decrypt entity_list (EntityRealm->entityList)
/// Returns Il2Cpp handle that needs Il2cppGetHandle() call
pub fn decrypt_entity_list(encrypted: u64) -> u64 {
    let mut value = encrypted;
    let chunks = unsafe { std::slice::from_raw_parts_mut(&mut value as *mut u64 as *mut u32, 2) };
    
    for i in 0..2 {
        let mut v = chunks[i];
        v = v.rotate_left(7);   // ROL 7
        v ^= 0x1714CF61;
        v = v.rotate_left(4);   // ROL 4
        chunks[i] = v;
    }
    
    value
}

/// Il2Cpp GCHandle->object resolver
/// Takes encrypted handle value and returns actual object pointer
pub fn il2cpp_gchandle_get_target(handle: u64, game_assembly_base: usize) -> u64 {
    // Call il2cpp_gchandle_get_target export
    // RVA: 0x81a840
    // You need to use driver or CreateRemoteThread to call this
    // For now, return raw handle (simplified)
    handle
}

// ============================================================================
// VECTOR MATH
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Matrix4x4 {
    pub m: [[f32; 4]; 4],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    
    pub fn distance(&self, other: &Vec3) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
    
    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    
    pub fn normalize(&self) -> Vec3 {
        let len = self.length();
        if len > 0.0 {
            Vec3 {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {
            *self
        }
    }
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

// ============================================================================
// WORLD TO SCREEN PROJECTION
// ============================================================================

/// World-to-screen using view matrix
/// Returns None if point is behind camera
pub fn world_to_screen(world_pos: Vec3, view_matrix: &Matrix4x4, screen_width: i32, screen_height: i32) -> Option<Vec2> {
    let m = &view_matrix.m;
    
    let w = m[3][0] * world_pos.x + m[3][1] * world_pos.y + m[3][2] * world_pos.z + m[3][3];
    
    if w < 0.001 {
        return None; // Behind camera
    }
    
    let x = m[0][0] * world_pos.x + m[0][1] * world_pos.y + m[0][2] * world_pos.z + m[0][3];
    let y = m[1][0] * world_pos.x + m[1][1] * world_pos.y + m[1][2] * world_pos.z + m[1][3];
    
    let screen_x = (screen_width as f32 / 2.0) * (1.0 + x / w);
    let screen_y = (screen_height as f32 / 2.0) * (1.0 - y / w);
    
    // Check if on screen
    if screen_x < 0.0 || screen_x > screen_width as f32 || screen_y < 0.0 || screen_y > screen_height as f32 {
        return None;
    }
    
    Some(Vec2 { x: screen_x, y: screen_y })
}

// ============================================================================
// IL2CPP TYPE DEFINITIONS
// ============================================================================

// Example of how to use type definitions from dump
pub const BASEPLAYER_TYPEDEFINITION_INDEX: usize = 5723;
pub const BASEPROJECTILE_TYPEDEFINITION_INDEX: usize = 520;
pub const BASECOMBATENTITY_TYPEDEFINITION_INDEX: usize = 5971;
pub const BASEENTITY_TYPEDEFINITION_INDEX: usize = 3676;

// Function RVAs you can call via driver
pub const BASEPLAYER_GET_HELDENTITY_RVA: usize = 0x3d94ff0;
pub const BASEPLAYER_GET_MOUNTED_RVA: usize = 0x3da9e60;
pub const BASEPROJECTILE_GETAIMCONE_RVA: usize = 0x34722c0;
pub const CAMERA_WORLDTOSCREEN_RVA: usize = 0x7a230;
