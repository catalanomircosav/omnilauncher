use crate::db::Game;
use super::LibraryParser;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::fs;
use std::path::PathBuf;

pub struct Shadps4Parser;

impl LibraryParser for Shadps4Parser {
    fn get_installed_games() -> Vec<Game> {
        let mut games = Vec::new();

        // We assume the user has make a global setup of ShadPS4 for now.
        // Retrieve the roaming AppData directory dynamically from Windows environment variables.
        // This automatically resolves to C:\Users\<Current_User>\AppData\Roaming
        if let Ok(appdata) = std::env::var("APPDATA") {
            let shadps4_root = PathBuf::from(appdata).join("shadPS4");
            let game_data_path = shadps4_root.join("game_data");

            if game_data_path.exists() {
                // Iterate through all Title ID folders (e.g., CUSA03173)
                if let Ok(entries) = fs::read_dir(&game_data_path) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        
                        if path.is_dir() {
                            // Build the path to the TROP.xml file
                            let trophy_xml_path = path
                                .join("TrophyFiles")
                                .join("trophy00")
                                .join("Xml")
                                .join("TROP.xml");

                            if trophy_xml_path.exists() {
                                // Safely extract the folder name (CUSA ID)
                                if let Some(folder_name) = path.file_name() {
                                    let cusa_id = folder_name.to_string_lossy().to_string();
                                    
                                    if let Some(game) = parse_trophy_xml(&trophy_xml_path, cusa_id) {
                                        games.push(game);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        games
    }
}

/// Parses the TROP.xml to extract the game title, ignoring trophy status.
fn parse_trophy_xml(xml_path: &PathBuf, cusa_id: String) -> Option<Game> {
    let mut reader = match Reader::from_file(xml_path) {
        Ok(r) => r,
        Err(_) => return None,
    };
    
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();

    let mut game_title = String::new();
    let mut current_text_tag = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Err(_) | Ok(Event::Eof) => break,

            // Start tags: <tag>
            Ok(Event::Start(ref e)) => {
                if e.name().as_ref() == b"title-name" {
                    current_text_tag = "title-name".to_string();
                } else {
                    current_text_tag.clear();
                }
            }

            // Text inside tags: <tag>TEXT</tag>
            Ok(Event::Text(e)) => {
                if current_text_tag == "title-name" {
                    // Extract text safely
                    game_title = String::from_utf8_lossy(e.as_ref()).into_owned();
                }
            }
            
            _ => (),
        }
        buf.clear();
    }

    // We only care that we found a valid title for the game folder
    if !game_title.is_empty() {
        Some(Game {
            id: None,
            description: None,
            title: game_title,
            platform: "PS4".to_string(),
            executable_path: cusa_id, 
            cover_url: None,
        })
    } else {
        None
    }
}