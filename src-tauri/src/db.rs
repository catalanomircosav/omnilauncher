use rusqlite::{Connection, Result, params};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: Option<i32>,
    pub description: Option<String>,
    pub title: String,
    pub platform: String,
    pub executable_path: String,
    pub cover_url: Option<String>,
}

/// Inserts a new game into the database
pub fn insert_game(conn: &Connection, game: &Game) -> Result<usize> {
    conn.execute(
        "INSERT INTO games (title, description, platform, executable_path, cover_url)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            &game.title,
            &game.description,
            &game.platform,
            &game.executable_path,
            &game.cover_url
        ],
    )
}

/// Retrieves all the games from the database
pub fn get_all_games(conn: &Connection) -> Result<Vec<Game>> {
    let mut stmt = conn.prepare("SELECT id, description, title, platform, executable_path, cover_url FROM games")?;

    let game_iter = stmt.query_map([], |row| {
        Ok(Game {
            id: row.get(0)?,
            description: row.get(1)?,
            title: row.get(2)?,
            platform: row.get(3)?,
            executable_path: row.get(4)?,
            cover_url: row.get(5)?,
        })
    })?;

    let mut games = Vec::new();
    for game in game_iter {
        games.push(game?);
    }

    Ok(games)
}

/// Initializes the SQLite database and creates the necessary tables
pub fn init_db(app_data_dir: &PathBuf) -> Result<Connection> {
    // Ensure the application data directory exists
    if !app_data_dir.exists() {
        fs::create_dir_all(app_data_dir).expect("Failed to create application data directory");
    }

    // Define the path to the local database
    let db_path = app_data_dir.join("omnilauncher.db");

    // Open the connection and create the file if it doesn't exist
    let conn = Connection::open(db_path)?;

    // Create the 'games' table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS games (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT,
            platform TEXT NOT NULL,
            executable_path TEXT NOT NULL,
            cover_url TEXT
        )",
        [], // Empty parameters
    )?;

    Ok(conn)
}