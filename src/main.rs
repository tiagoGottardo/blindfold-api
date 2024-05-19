#![allow(unused)]

use axum::extract::{Json, Request};
use axum::response::{Html, IntoResponse};
use serde::Deserialize;
use tokio::net::TcpListener;

use axum::routing::{get, post};
use axum::Router;

#[derive(Debug, Deserialize)]
struct InitializeGameRequest {
    color: Option<String>,
    start_position: Option<String>,
    stockfish_level: Option<usize>,
}

const FEN_START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[tokio::main]
async fn main() {
    let routes = Router::new()
        .route("/health", get(|| async { "It's okay" }))
        .route("/newgame", post(new_game_handler));

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();

    axum::serve(listener, routes).await.unwrap();
}

async fn new_game_handler(Json(mut body): Json<InitializeGameRequest>) -> impl IntoResponse {
    println!("{:?}", body);

    Html(format!(
        "<h1>A new game was initialized, you're {} against stockfish level {} on position {}</h1>",
        body.color.unwrap_or("white".to_string()),
        body.stockfish_level.unwrap_or(2),
        body.start_position
            .unwrap_or(FEN_START_POSITION.to_string())
    ))
}
