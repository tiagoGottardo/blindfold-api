const FEN_START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

use std::sync::Arc;
use tokio::sync::Mutex;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct InitializeGameRequest {
    color: Option<String>,
    start_position: Option<String>,
    stockfish_level: Option<u8>,
}

#[derive(Debug, Deserialize)]
pub struct PlayMoveRequest {
    pub id: Uuid,
    pub play: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct Game {
    pub id: Uuid,
    pub player_color: String,
    pub start_position: String,
    pub actual_position: String,
    pub moves: Vec<[String; 2]>,
    pub full_moves_count: u8,
    pub stockfish_level: u8,
    pub finalized: bool,
}

pub type DB = Arc<Mutex<Vec<Game>>>;

pub fn create_db() -> DB {
    Arc::new(Mutex::new(Vec::new()))
}

impl Game {
    pub fn new(req: InitializeGameRequest) -> Self {
        Self {
            id: Uuid::new_v4(),
            player_color: req.color.unwrap_or("white".to_string()),
            full_moves_count: 1,
            start_position: req
                .start_position
                .clone()
                .unwrap_or(FEN_START_POSITION.to_string()),
            actual_position: req.start_position.unwrap_or(FEN_START_POSITION.to_string()),
            moves: Vec::new(),
            stockfish_level: req.stockfish_level.unwrap_or(4),
            finalized: false,
        }
    }
}
