#![allow(unused)]
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use axum::extract::{Json, Request, State};
use axum::response::{Html, IntoResponse};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

use axum::routing::{get, post};
use axum::Router;
use uuid::uuid;

#[derive(Debug, Deserialize)]
struct InitializeGameRequest {
    color: Option<String>,
    start_position: Option<String>,
    stockfish_level: Option<u8>,
}

#[derive(Serialize, Clone, Debug)]
pub struct Game {
    id: Uuid,
    player_color: String,
    start_position: String,
    actual_position: String,
    moves: Vec<String>,
    stockfish_level: u8,
    finalized: bool,
}

impl Game {
    fn new(req: InitializeGameRequest) -> Self {
        Self {
            id: Uuid::new_v4(),
            player_color: req.color.unwrap_or("white".to_string()),
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

pub type DB = Arc<Mutex<Vec<Game>>>;

pub fn create_db() -> DB {
    Arc::new(Mutex::new(Vec::new()))
}

const FEN_START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[tokio::main]
async fn main() {
    let db = create_db();

    let routes = Router::new()
        .route("/health", get(|| async { "It's okay" }))
        .route("/newgame", post(new_game_handler))
        .with_state(db);

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();

    axum::serve(listener, routes).await.unwrap();
}

async fn new_game_handler(
    State(db): State<DB>,
    Json(mut body): Json<InitializeGameRequest>,
) -> impl IntoResponse {
    let mut vec = db.lock().await;

    let game = Game::new(body);

    vec.push(game.clone());

    Html(format!("{:#?}", game))
}
