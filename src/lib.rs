mod minimax;

use chess::{Board, ChessMove, Piece, Rank, Square};
use minimax::MiniMax;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

// interface
#[wasm_bindgen]
pub struct Chess {
    board: Board,
}

#[wasm_bindgen]
impl Chess {
    pub fn new() -> Chess {
        Chess {
            board: Board::default(),
        }
    }

    pub fn to_fen(&self) -> String {
        self.board.to_string()
    }

    pub fn validate_move(&self, chess_move: Vec<String>) -> bool {
        let from = Square::from_str(&chess_move[0]).unwrap_or_default();
        let to = Square::from_str(&chess_move[1]).unwrap_or_default();

        let m: ChessMove;

        if self.board.piece_on(from) == Some(Piece::Pawn) && to.get_rank() == Rank::Eighth {
            m = ChessMove::new(from, to, Some(Piece::Queen));
        } else {
            m = ChessMove::new(from, to, None);
        }

        self.board.legal(m)
    }

    pub fn make_move(&mut self, chess_move: Vec<String>) -> String {
        let from = Square::from_str(&chess_move[0]).unwrap_or_default();
        let to = Square::from_str(&chess_move[1]).unwrap_or_default();

        let m: ChessMove;

        if self.board.piece_on(from) == Some(Piece::Pawn) && to.get_rank() == Rank::First
            || to.get_rank() == Rank::Eighth
        {
            m = ChessMove::new(from, to, Some(Piece::Queen));
        } else {
            m = ChessMove::new(from, to, None);
        }

        let mut result = self.board.clone();
        self.board.make_move(m, &mut result);

        self.board = result;

        return format!("{:?}", self.board.status());
    }

    pub fn make_engine_move(&mut self) -> String {
        let m = MiniMax::eval_best_move(&self.board, 3);

        let mut result = self.board.clone();
        self.board.make_move(m, &mut result);
        self.board = result;

        return format!("{:?}", self.board.status());
    }

    pub fn reset(&mut self) {
        self.board = Board::default();
    }
}
