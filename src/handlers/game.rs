use crate::models::game::*;
use chess::{Board, ChessMove};
use std::str::FromStr;

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

fn mock_stockfish(board_state: String) -> String {
    todo!();
}

pub async fn play_move(
    State(db): State<DB>,
    Json(body): Json<PlayMoveRequest>,
) -> impl IntoResponse {
    let vec = db.lock().await;

    let mut game = match vec.iter().find(|game| game.id == body.id) {
        Some(game) => game.clone(),
        None => return Html("Game not found!"),
    };

    let board = match Board::from_str(&game.actual_position) {
        Ok(game) => game,
        Err(err) => return Html(format!("Game is invalid! | {}", err).leak()),
    };
    let new_fen: String;

    let chess_move = match ChessMove::from_san(&board, body.play.as_str()) {
        Ok(play) => {
            let mut new_board = Board::default();
            board.make_move(play, &mut new_board);
            new_fen = board.to_string();
            play.to_string()
        }
        Err(err) => return Html(format!("Illegal Move! | {}", err).leak()),
    };

    if game.player_color == "white" {
        game.moves.push([body.play, "".to_string()]);
    } else {
        if let Some(last) = game.moves.last_mut() {
            last[1] = String::from(body.play);
        } else {
            game.moves.push(["".to_string(), body.play]);
        }
    }

    game.actual_position = new_fen;

    let stockfish_move = mock_stockfish(chess_move);

    Html(format!("Game is on!: {:#?}", game).leak())
}
