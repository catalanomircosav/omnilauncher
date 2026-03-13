use crate::db::Game;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

use super::LibraryParser;

pub struct SteamParser;

impl LibraryParser for SteamParser {
    fn get_installed_games() -> Vec<Game> {
        let mut games: Vec<Game> = Vec::new();

        // 1. Orchestrate the path discovery entirely within the trait method
        if let Some(steam_path) = get_steam_install_path() {
            let vdf_path = get_library_folders_path(&steam_path);
            let library_paths = parse_library_paths(&vdf_path);

            // 2. Scan all discovered libraries
            for lib_path in library_paths {
                let steamapps_path = lib_path.join("steamapps");

                if let Ok(entries) = fs::read_dir(&steamapps_path) {
                    for entry in entries.flatten() {
                        let path = entry.path();

                        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("acf") {
                            if let Some(game) = parse_appmanifest(&path) {
                                games.push(game);
                            }
                        }
                    }
                }
            }
        }

        games
    }
}

/// Retrieves the base installation path of Steam, currently only for Windows.
#[cfg(target_os = "windows")]
fn get_steam_install_path() -> Option<PathBuf> {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    if let Ok(steam_key) = hkcu.open_subkey("Software\\Valve\\Steam") {
        if let Ok(path) = steam_key.get_value::<String, _>("SteamPath") {
            return Some(PathBuf::from(path));
        }
    }

    None
}

#[cfg(not(target_os = "windows"))]
fn get_steam_install_path() -> Option<PathBuf> {
    None
}

fn get_library_folders_path(steam_path: &PathBuf) -> PathBuf {
    steam_path.join("steamapps/libraryfolders.vdf")
}

/// Parses the libraryfolders.vdf file to extract all Steam library paths.
fn parse_library_paths(vdf_path: &PathBuf) -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Ok(file) = File::open(vdf_path) {
        let reader = io::BufReader::new(file);

        for line in reader.lines().flatten() {
            let trimmed = line.trim();
            
            if trimmed.starts_with("\"path\"") {
                let parts: Vec<&str> = trimmed.split('\"').collect();
                if parts.len() >= 4 {
                    // Steam escapes backslashes, we need to clean them up for PathBuf
                    let clean_path = parts[3].replace("\\\\", "\\");
                    paths.push(PathBuf::from(clean_path));
                }
            }
        }
    }

    paths
}

fn parse_appmanifest(path: &PathBuf) -> Option<Game> {
    let mut appid = String::new();
    let mut name = String::new();

    if let Ok(file) = File::open(path) {
        let reader = io::BufReader::new(file);

        for line in reader.lines().flatten() {
            let trimmed = line.trim();

            if trimmed.starts_with("\"appid\"") { 
                let parts: Vec<&str> = trimmed.split('\"').collect();
                if parts.len() >= 4 {
                    appid = parts[3].to_string();
                }
            } else if trimmed.starts_with("\"name\"") {
                let parts: Vec<&str> = trimmed.split('\"').collect();
                if parts.len() >= 4 {
                    name = parts[3].to_string();
                }
            }

            if !appid.is_empty() && !name.is_empty() { break; }
        }
    }
    
    if !appid.is_empty() && !name.is_empty() { 
        if !is_valid_game(&appid, &name) {
            return None;
        }

        Some(Game {
            id: None,
            description: None,
            title: name,
            platform: "Steam".to_string(),
            executable_path: appid,
            cover_url: None,
        })
    } else { 
        None 
    }
}

/// Helper to ignore non valid apps
fn is_valid_game(appid: &str, name: &str) -> bool {
    let ignored_appids = ["228980"];
    if ignored_appids.contains(&appid) {
        return false;
    }

    let name_lower = name.to_lowercase();
    let ignored_keywords = [
        "steamworks",
        "proton",
        "steam linux runtime",
        "soundtrack",
        "dedicated server",
        "sdk"
    ];

    for keyword in ignored_keywords.iter() {
        if name_lower.contains(keyword) {
            return false;
        }
    }

    true
}