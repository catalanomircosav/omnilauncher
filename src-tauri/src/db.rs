use rusqlite::{Connection, Result, params};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: Option<i32>,
    pub game_id: String,
    pub description: Option<String>,
    pub title: String,
    pub platform: String,
    pub executable_path: String,
    pub cover_url: Option<String>,
}

/// Inserts a new game into the database
pub fn insert_game(conn: &Connection, game: &Game) -> Result<usize> {

    let exists: bool = conn.query_row(
    "SELECT EXISTS(SELECT 1 FROM games WHERE title = ?1 AND platform = ?2)",
        [&game.title, &game.platform],
        |row| row.get(0),
    )?;

    if exists {
        println!("Game already exists: {} [{}]", game.title, game.platform);
        return Ok(0);
    } else {
        conn.execute(
            "INSERT INTO games (title, game_id, description, platform, executable_path, cover_url)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                &game.title,
                &game.game_id,
                &game.description,
                &game.platform,
                &game.executable_path,
                &game.cover_url
            ],
        )
    }
}

/// Retrieves all the games from the database
pub fn _get_all_games(conn: &Connection) -> Result<Vec<Game>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT id, game_id, description, title, platform, executable_path, cover_url FROM games")?;

    let game_iter = stmt.query_map([], |row| {
        Ok(Game {
            id: row.get(0)?,
            game_id: row.get(1)?,
            description: row.get(2)?,
            title: row.get(3)?,
            platform: row.get(4)?,
            executable_path: row.get(5)?,
            cover_url: row.get(6)?,
        })
    })?;

    game_iter.collect()
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
            game_id TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            platform TEXT NOT NULL,
            executable_path TEXT,
            cover_url TEXT
        )",
        [], // Empty parameters
    )?;

    Ok(conn)
}