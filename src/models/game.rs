const FEN_START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

use axum::{http::StatusCode, response::IntoResponse, Json};
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

pub struct ErrorDefault {
    code: StatusCode,
    message: String,
}

impl ErrorDefault {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

#[derive(Serialize)]
struct ResponseMessage {
    message: String,
}

impl IntoResponse for ErrorDefault {
    fn into_response(self) -> axum::response::Response {
        (
            self.code,
            Json(ResponseMessage {
                message: self.message,
            }),
        )
            .into_response()
    }
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

pub async fn create_db() -> DB {
    let db = Arc::new(Mutex::new(Vec::new()));
    let unique = Game {
        player_color: "black".to_string(),
        id: Uuid::parse_str("e06fd117-8ab1-4b3d-9203-ae277de29347").unwrap(),
        full_moves_count: 1,
        start_position: FEN_START_POSITION.to_string(),
        actual_position: FEN_START_POSITION.to_string(),
        moves: Vec::new(),
        stockfish_level: 4,
        finalized: false,
    };
    db.lock().await.push(unique);
    db
}

impl Game {
    pub fn new(req: InitializeGameRequest) -> Self {
        let color = match req.color {
            Some(color) if color == "black" => "b",
            _ => "w",
        };
        Self {
            id: Uuid::new_v4(),
            player_color: String::from(color),
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
