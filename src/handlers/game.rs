use crate::models::game::*;

use axum::extract::{Json, State};
use axum::response::{Html, IntoResponse};

pub async fn new_game_handler(
    State(db): State<DB>,
    Json(body): Json<InitializeGameRequest>,
) -> impl IntoResponse {
    let mut vec = db.lock().await;

    let game = Game::new(body);

    vec.push(game.clone());

    Html(format!("{:#?}", game))
}
