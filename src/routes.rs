use crate::handlers::game::*;
use axum::routing::{get, post};
use axum::Router;

use crate::create_db;

pub fn create_router() -> Router {
    let db = create_db();

    Router::new()
        .route("/health", get(|| async { "It's okay" }))
        .route("/newgame", post(new_game_handler))
        .with_state(db)
}

