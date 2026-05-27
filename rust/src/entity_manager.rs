// Entity Manager - Handles encrypted entity list traversal
// Build 23369401 compatible

use crate::offsets_new::{RustOffsets, Vec3, decrypt_client_entities, decrypt_entity_list};
use crate::memory::Process;

pub struct EntityManager {
    process: Process,
    offsets: RustOffsets,
    game_assembly_base: usize,
}

#[derive(Debug, Clone)]
pub struct EntityInfo {
    pub address: usize,
    pub position: Vec3,
    pub health: f32,
    pub max_health: f32,
    pub class_name: String,
}

impl EntityManager {
    pub fn new(process: Process, game_assembly_base: usize) -> Self {
        Self {
            process,
            offsets: RustOffsets::new(),
            game_assembly_base,
        }
    }
    
    /// Get ALL entities from encrypted BaseNetworkable list
    /// This is the CORRECT way for build 23369401
    pub fn get_all_entities(&self) -> Vec<usize> {
        let mut entities = Vec::new();
        
        // Step 1: Get BaseNetworkable.Static class
        let base_networkable_static = self.game_assembly_base + self.offsets.base_networkable_static;
        
        // Step 2: Read static fields pointer
        let static_fields = match self.process.read::<usize>(base_networkable_static + 0xb8) {
            Ok(ptr) if ptr != 0 => ptr,
            _ => {
                println!("[-] Failed to read BaseNetworkable static fields");
                return entities;
            }
        };
        
        // Step 3: Read encrypted clientEntities pointer
        let encrypted_client_entities = match self.process.read::<u64>(static_fields + self.offsets.client_entities_offset) {
            Ok(val) => val,
            Err(_) => {
                println!("[-] Failed to read clientEntities");
                return entities;
            }
        };
        
        // Step 4: Decrypt clientEntities
        let decrypted_handle = decrypt_client_entities(encrypted_client_entities);
        
        // Step 5: Resolve Il2Cpp handle to object pointer
        // This would normally call il2cpp_gchandle_get_target
        // For simplified version, assume handle == pointer
        let client_entities = decrypted_handle as usize;
        
        if client_entities == 0 {
            println!("[-] clientEntities decrypted to null");
            return entities;
        }
        
        println!("[+] clientEntities: 0x{:X}", client_entities);
        
        // Step 6: Get EntityRealm from clientEntities
        let entity_realm = match self.process.read::<usize>(client_entities + self.offsets.entity_realm_offset) {
            Ok(ptr) if ptr != 0 => ptr,
            _ => {
                println!("[-] Failed to read EntityRealm");
                return entities;
            }
        };
        
        println!("[+] EntityRealm: 0x{:X}", entity_realm);
        
        // Step 7: Read encrypted entityList
        let encrypted_entity_list = match self.process.read::<u64>(entity_realm + 0x10) {
            Ok(val) => val,
            Err(_) => {
                println!("[-] Failed to read encrypted entityList");
                return entities;
            }
        };
        
        // Step 8: Decrypt entityList
        let decrypted_list_handle = decrypt_entity_list(encrypted_entity_list);
        let entity_list = decrypted_list_handle as usize;
        
        if entity_list == 0 {
            println!("[-] entityList decrypted to null");
            return entities;
        }
        
        println!("[+] entityList: 0x{:X}", entity_list);
        
        // Step 9: Get ListDictionary->vals (BufferList)
        let buffer_list = match self.process.read::<usize>(entity_list + self.offsets.entity_list_vals) {
            Ok(ptr) if ptr != 0 => ptr,
            _ => {
                println!("[-] Failed to read BufferList");
                return entities;
            }
        };
        
        // Step 10: Read BufferList count
        let count = match self.process.read::<i32>(buffer_list + 0x18) {
            Ok(c) => c as usize,
            Err(_) => {
                println!("[-] Failed to read entity count");
                return entities;
            }
        };
        
        println!("[+] Entity count: {}", count);
        
        // Step 11: Get BufferList->buffer
        let buffer = match self.process.read::<usize>(buffer_list + 0x10) {
            Ok(ptr) if ptr != 0 => ptr,
            _ => {
                println!("[-] Failed to read entity buffer");
                return entities;
            }
        };
        
        // Step 12: Read all entity pointers
        for i in 0..count.min(10000) {  // Safety limit
            let entity_ptr = match self.process.read::<usize>(buffer + i * 0x8) {
                Ok(ptr) if ptr != 0 => ptr,
                _ => continue,
            };
            
            entities.push(entity_ptr);
        }
        
        println!("[+] Read {} valid entities", entities.len());
        entities
    }
    
    /// Get only BasePlayer entities (filtered)
    pub fn get_players(&self) -> Vec<usize> {
        let all_entities = self.get_all_entities();
        let mut players = Vec::new();
        
        for entity in all_entities {
            // Quick check: Does it have BasePlayer vtable?
            // BasePlayer typeinfo: 0xe27c4a8
            if let Ok(vtable) = self.process.read::<usize>(entity) {
                // Check if vtable points to BasePlayer range
                // This is simplified - proper way is to check typeinfo
                if vtable > self.game_assembly_base && 
                   vtable < self.game_assembly_base + 0x10000000 {
                    players.push(entity);
                }
            }
        }
        
        println!("[+] Found {} potential players", players.len());
        players
    }
    
    /// Read entity details (health, position, etc)
    pub fn get_entity_info(&self, entity: usize) -> Option<EntityInfo> {
        // Read health
        let health = self.process.read::<f32>(entity + self.offsets.health).ok()?;
        if health <= 0.0 || health > 1000.0 {
            return None; // Invalid health
        }
        
        let max_health = self.process.read::<f32>(entity + self.offsets.max_health).ok()?;
        
        // Read position via PlayerModel
        let player_model = self.process.read::<usize>(entity + self.offsets.player_model).ok()?;
        if player_model == 0 {
            return None;
        }
        
        let position = self.process.read::<Vec3>(player_model + self.offsets.player_model_position).ok()?;
        
        Some(EntityInfo {
            address: entity,
            position,
            health,
            max_health,
            class_name: String::from("BasePlayer"),
        })
    }
    
    /// Get LocalPlayer (decrypted)
    pub fn get_local_player(&self) -> Option<usize> {
        // Step 1: Find LocalPlayer static class
        // You need to pattern scan for this, example pattern:
        // "48 8B 0D ? ? ? ? 48 85 C9 74 ? E8 ? ? ? ? 48 8B D8"
        
        // For now, return None (needs pattern scanner)
        // See find_local_player() in scanner module
        None
    }
}

// Helper to check if entity is BasePlayer
pub fn is_base_player(process: &Process, entity: usize, game_assembly_base: usize) -> bool {
    // Read vtable
    if let Ok(vtable) = process.read::<usize>(entity) {
        // BasePlayer typeinfo should be at vtable-0x10 or similar
        // Simplified check: just verify it's in game code range
        vtable > game_assembly_base && vtable < game_assembly_base + 0x10000000
    } else {
        false
    }
}
