use std::env;
use std::fs;
use std::path::PathBuf;

/// Windows build workaround for usearch MAP_FAILED issue
/// See: https://github.com/jxoesneon/mempalace-rs/issues/3
/// Upstream issue: https://github.com/unum-cloud/USearch/issues/746
fn main() {
    // Only apply patch on Windows
    if env::var("TARGET")
        .map(|t| t.contains("windows"))
        .unwrap_or(false)
    {
        patch_usearch_for_windows();
    }

    // Rerun if Cargo.toml changes
    println!("cargo:rerun-if-changed=Cargo.toml");
}

fn patch_usearch_for_windows() {
    // Find the usearch crate in cargo registry cache
    let home = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());

    let cargo_registry = PathBuf::from(&home)
        .join(".cargo")
        .join("registry")
        .join("src");

    if !cargo_registry.exists() {
        // Registry might not exist yet, skip patching
        return;
    }

    // Search for usearch-* directories
    let entries = match fs::read_dir(&cargo_registry) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        if !entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
            continue;
        }

        // Look for usearch directories inside registry index folders
        let subdir = entry.path();
        let subentries = match fs::read_dir(&subdir) {
            Ok(e) => e,
            Err(_) => continue,
        };

        for subentry in subentries {
            let subentry = match subentry {
                Ok(e) => e,
                Err(_) => continue,
            };

            let name = subentry.file_name();
            let name_str = name.to_string_lossy();

            if name_str.starts_with("usearch-") {
                let header_path = subentry
                    .path()
                    .join("include")
                    .join("usearch")
                    .join("index_plugins.hpp");

                if header_path.exists() {
                    apply_map_failed_patch(&header_path);
                }
            }
        }
    }
}

fn apply_map_failed_patch(header_path: &PathBuf) {
    let content = match fs::read_to_string(header_path) {
        Ok(c) => c,
        Err(_) => return,
    };

    // Check if already patched
    if content.contains("#ifndef MAP_FAILED") {
        return;
    }

    // Apply the MAP_FAILED patch for Windows
    // Find a good location to insert (after includes, before first use)
    let patch = r#"
// Windows MAP_FAILED workaround - see https://github.com/unum-cloud/USearch/issues/746
#ifndef MAP_FAILED
#define MAP_FAILED ((void*)-1)
#endif
"#;

    // Insert after the first #include or at the top
    let patched_content = if let Some(pos) = content.find("#pragma once") {
        let insert_pos = content[pos..]
            .find('\n')
            .map(|p| pos + p + 1)
            .unwrap_or(pos + 12);
        let mut new_content = content.clone();
        new_content.insert_str(insert_pos, patch);
        new_content
    } else {
        format!("{}{}", patch, content)
    };

    if let Err(e) = fs::write(header_path, patched_content) {
        eprintln!("Warning: Failed to patch usearch header: {}", e);
    } else {
        println!(
            "cargo:warning=Patched usearch header for Windows: {:?}",
            header_path
        );
    }
}
