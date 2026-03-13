use crate::db::Game;

pub mod steam;

pub trait LibraryParser {
    fn get_installed_games() -> Vec<Game>;
}