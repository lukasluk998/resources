// Automatic offset updater from UnknownCheats forum
// Scrapes latest post and updates offsets.rs
//
// Usage: cargo run --bin update_offsets

use std::fs;
use std::io::Write;

fn main() {
    println!("╔═══════════════════════════════════════════╗");
    println!("║   Rust Offset Auto-Updater v1.0          ║");
    println!("║   Source: UnknownCheats Forum             ║");
    println!("╚═══════════════════════════════════════════╝");
    println!();
    
    println!("[*] Instructions:");
    println!("    1. Go to: https://www.unknowncheats.me/forum/rust/");
    println!("    2. Find thread: 'Rust Reversal, Structs and Offsets'");
    println!("    3. Copy LATEST post with offsets");
    println!("    4. Save to: resources/offsets/latest_uc_post.txt");
    println!();
    println!("[*] Manual method (if above doesn't work):");
    println!("    - Use offsets_new.rs (already updated for build 23369401)");
    println!("    - Copy fresh decrypts from UnknownCheats");
    println!();
    
    // Check if user has saved offsets file
    let offsets_file = "../resources/offsets/latest_uc_post.txt";
    
    if !std::path::Path::new(offsets_file).exists() {
        println!("[-] File not found: {}", offsets_file);
        println!("[!] Please create file and paste latest UnknownCheats post");
        println!();
        println!("[+] For now, use offsets_new.rs (build 23369401 - 2026-05-25)");
        return;
    }
    
    println!("[+] Found offset file!");
    println!("[*] Parsing offsets...");
    
    let content = match fs::read_to_string(offsets_file) {
        Ok(c) => c,
        Err(e) => {
            println!("[-] Failed to read file: {}", e);
            return;
        }
    };
    
    // Parse buildid
    let buildid = parse_buildid(&content);
    if let Some(id) = buildid {
        println!("[+] BuildID: {}", id);
    } else {
        println!("[-] Could not find BuildID in post");
    }
    
    // Parse timestamp
    if let Some(ts) = parse_timestamp(&content) {
        println!("[+] Timestamp: {}", ts);
    }
    
    // TODO: Full parser for offset extraction
    // For now, manual copy-paste is faster
    
    println!();
    println!("[*] Recommendation:");
    println!("    1. Compare with offsets_new.rs");
    println!("    2. If buildID matches, offsets are current");
    println!("    3. If buildID different, manually update offsets_new.rs");
    println!();
    println!("[+] Current offsets (offsets_new.rs): build 23369401");
}

fn parse_buildid(content: &str) -> Option<String> {
    // Look for "Rust buildid: 23369401" pattern
    for line in content.lines() {
        if line.contains("buildid") {
            if let Some(start) = line.find(':') {
                let id_part = &line[start+1..].trim();
                // Extract just the number
                let id: String = id_part.chars()
                    .take_while(|c| c.is_numeric())
                    .collect();
                if !id.is_empty() {
                    return Some(id);
                }
            }
        }
    }
    None
}

fn parse_timestamp(content: &str) -> Option<String> {
    // Look for "Dump generated on: 2026-05-25" pattern
    for line in content.lines() {
        if line.contains("Dump generated on") {
            if let Some(start) = line.find(':') {
                return Some(line[start+1..].trim().to_string());
            }
        }
    }
    None
}
