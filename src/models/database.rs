const FEN_START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

use crate::models::game::*;
use uuid::Uuid;

use std::sync::Arc;
use tokio::sync::Mutex;

pub type DB = Arc<Mutex<Vec<Game>>>;

pub async fn create_db() -> DB {
    let db = Arc::new(Mutex::new(Vec::new()));
    let unique = Game {
        player_color: PlayerColor::Black,
        id: Uuid::parse_str("e06fd117-8ab1-4b3d-9203-ae277de29347").unwrap(),
        full_moves_count: 1,
        start_position: FEN_START_POSITION.to_string(),
        actual_position: FEN_START_POSITION.to_string(),
        moves: Vec::new(),
        stockfish_level: 1,
        finalized: false,
    };
    db.lock().await.push(unique);
    db
}
