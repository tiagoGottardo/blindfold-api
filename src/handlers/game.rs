use crate::models::database::DB;
use crate::models::error::ErrorDefault;
use crate::models::game::*;
use axum::http::StatusCode;
use chess::{Board, ChessMove};
use std::io::Write;
use std::process::{Command, Stdio};
use std::str::FromStr;

use axum::extract::{Json, State};
use axum::response::IntoResponse;

pub async fn new_game_handler(
    State(db): State<DB>,
    Json(body): Json<InitializeGameRequest>,
) -> impl IntoResponse {
    let mut vec = db.lock().await;

    let game = Game::new(body);

    vec.push(game.clone());

    Json(game).into_response()
}

fn remove_range(s: &str, start: usize, end: usize) -> String {
    if start >= end || start >= s.len() || end > s.len() {
        return s.to_string();
    }

    let byte_start = s
        .char_indices()
        .nth(start)
        .map(|(i, _)| i)
        .unwrap_or(s.len());
    let byte_end = s.char_indices().nth(end).map(|(i, _)| i).unwrap_or(s.len());

    let before = &s[..byte_start];
    let after = &s[byte_end..];

    format!("{}{}", before, after)
}

fn make_san_move(fen: &str, san_move: &str) -> Result<String, ErrorDefault> {
    let board = Board::from_str(fen).or_else(|e| {
        Err(ErrorDefault::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        ))
    })?;

    let chess_move = ChessMove::from_san(&board, san_move).or_else(|e| {
        Err(ErrorDefault::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        ))
    })?;
    let mut new_board = Board::default();
    board.make_move(chess_move, &mut new_board);

    Ok(new_board.to_string())
}

fn make_lan_move(fen: &str, lan_move: &str) -> Result<(String, String), ErrorDefault> {
    let mut moves: Vec<String> = vec![];

    if lan_move.chars().next().unwrap().is_lowercase() {
        if lan_move.chars().any(|c| c == 'x') {
            moves.push(remove_range(lan_move, 1, 2));
        } else {
            moves.push(remove_range(lan_move, 0, 2));
        }
    } else {
        moves.push(remove_range(lan_move, 1, 3));
        moves.push(remove_range(lan_move, 2, 3));
        moves.push(remove_range(lan_move, 1, 2));
    }

    let board = Board::from_str(&fen).or_else(|e| {
        Err(ErrorDefault::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        ))
    })?;

    let mut san_notation_play = String::new();
    let mut new_fen: String = String::new();
    for play in moves.iter() {
        match ChessMove::from_san(&board, play) {
            Ok(play_move) => {
                let mut new_board = Board::default();
                san_notation_play = play.to_string();
                board.make_move(play_move, &mut new_board);
                new_fen = new_board.to_string();
                break;
            }
            Err(_) => {
                continue;
            }
        };
    }

    Ok((san_notation_play, new_fen))
}

fn add_to_move_list(list: &mut Vec<[String; 2]>, play: &str, color: &str) {
    if color == "w" {
        list.push([String::from(play), String::from("")]);
    } else {
        if let Some(last) = list.last_mut() {
            last[1] = String::from(play);
        } else {
            list.push([String::from(""), String::from(play)]);
        }
    }
}

fn stockfish_analizer(board_state: &str, stockfish_level: u8) -> Result<String, ErrorDefault> {
    let stockfish_path = "/usr/bin/stockfish";

    let mut child = Command::new(stockfish_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let stdin = match child.stdin.as_mut() {
        Some(std) => std,
        _ => {
            return Err(ErrorDefault::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Stockfish not found!",
            ))
        }
    };

    writeln!(stdin, "uci")?;
    writeln!(
        stdin,
        "setoption name Skill Level value {}",
        stockfish_level
    )?;
    writeln!(stdin, "position fen {}", board_state)?;
    writeln!(stdin, "go movetime 2000")?;

    let output = child.wait_with_output()?;

    let output_str = String::from_utf8_lossy(&output.stdout);

    let best_move = output_str
        .lines()
        .find(|line| line.starts_with("bestmove"))
        .unwrap_or("bestmove none");

    Ok(best_move
        .split_whitespace()
        .last()
        .unwrap_or("none")
        .to_string())
}

pub async fn play_move(
    State(db): State<DB>,
    Json(body): Json<PlayMoveRequest>,
) -> Result<impl IntoResponse, ErrorDefault> {
    let mut vec = db.lock().await;

    let mut game = match vec.iter().find(|game| game.id == body.id) {
        Some(game) => game.clone(),
        None => return Err(ErrorDefault::new(StatusCode::NOT_FOUND, "Game not found!")),
    };

    let play = body.play.clone();

    add_to_move_list(&mut game.moves, &play, &game.player_color);

    let new_fen = make_san_move(&game.actual_position, &body.play)?;

    let stockfish_move = stockfish_analizer(&new_fen, game.stockfish_level)?;

    let (san_stockfish_move, new_fen) = make_lan_move(&new_fen, &stockfish_move)?;

    add_to_move_list(
        &mut game.moves,
        &san_stockfish_move,
        if game.player_color == "w" { "b" } else { "w" },
    );

    game.actual_position = new_fen;
    game.full_moves_count += 1;

    match vec.iter_mut().find(|gamedb| gamedb.id == game.id) {
        Some(gamedb) => *gamedb = game,
        None => {
            return Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Something bad happen internally!"),
            ))
        }
    };

    Ok((
        StatusCode::OK,
        format!("Stockfish play: {}", san_stockfish_move),
    ))
}
