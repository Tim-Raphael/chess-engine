use chess::{Board, BoardStatus, Color, MoveGen, Piece, Square};
use std::str::FromStr;

pub struct MiniMax;

impl MiniMax {
    pub fn eval_best_move(board: &Board, depth: u8) -> chess::ChessMove {
        let mut best_move = chess::ChessMove::default();
        let mut best_score = i32::MIN;

        let color = board.side_to_move();

        for mv in MoveGen::new_legal(&board) {
            let mut result = board.clone();
            board.make_move(mv, &mut result);

            let score = MiniMax::minimax(&result, depth, i32::MIN, i32::MAX, false, color);
            if score > best_score {
                best_score = score;
                best_move = mv;
            }
        }

        best_move
    }

    pub fn evaluation(board: &Board, color: Color) -> i32 {
        let mut score = 0;

        for i in 0..64 {
            let squares = [
                "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "a2", "b2", "c2", "d2", "e2", "f2",
                "g2", "h2", "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "a4", "b4", "c4", "d4",
                "e4", "f4", "g4", "h4", "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", "a6", "b6",
                "c6", "d6", "e6", "f6", "g6", "h6", "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
                "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
            ];

            let square_values = [
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 3,
                3, 2, 1, 1, 1, 1, 2, 3, 3, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1,
            ];

            let square = Square::from_str(squares[i]).unwrap_or_default();
            let square_value = square_values[i];
            let piece = board.piece_on(square);

            if piece.is_none() {
                continue;
            }

            let piece = piece.unwrap();
            let value = match piece {
                Piece::Pawn => 100,
                Piece::Bishop => 300,
                Piece::Knight => 310, // knights are slightly more valuable in the center
                Piece::Rook => 500,
                Piece::Queen => 900,
                Piece::King => 2000,
            };

            let piece_color = board.color_on(square).unwrap();
            let sign = if piece_color == color { 1 } else { -1 };

            if piece == Piece::King {
                score += value * sign;
            } else {
                score += value * square_value * sign;
            }
        }

        match board.status() {
            BoardStatus::Checkmate => {
                if board.side_to_move() == Color::Black {
                    score -= 1_000_000;
                } else {
                    score += 1_000_000;
                }
            }
            _ => score += 0,
        }

        score
    }

    pub fn minimax(
        board: &Board,
        depth: u8,
        mut alpha: i32,
        mut beta: i32,
        is_maximizing: bool,
        color: Color
    ) -> i32 {
        if depth == 0 {
            return MiniMax::evaluation(board, color);
        }

        if is_maximizing {
            let mut best_score = -100_000_000;

            for mv in MoveGen::new_legal(&board) {
                let mut result = board.clone();
                board.make_move(mv, &mut result);

                let score = MiniMax::minimax(&result, depth - 1, alpha, beta,  !is_maximizing, color);

                best_score = best_score.max(score);

                if score > best_score {
                    best_score = score;
                } 

                if best_score > alpha {
                    alpha = score;
                }

                if beta <= alpha {
                    return score;
                }
            }

            best_score
        } else {
            let mut best_score = 100_000_000;

            for mv in MoveGen::new_legal(&board) {
                let mut result = board.clone();
                board.make_move(mv, &mut result);

                let score = MiniMax::minimax(&result, depth - 1, alpha, beta, !is_maximizing, color);

                if score < best_score {
                    best_score = score;
                } 

                if best_score < beta {
                    beta = score;
                }

                if beta <= alpha {
                    return score;
                }
            }

            best_score
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimax() {
        let board = Board::default();
        let color = Color::White;
        let depth = 3;
        let best_move = MiniMax::minimax(&board, depth, i32::MIN, i32::MAX, true, color);
        assert_eq!(best_move >= 0, true);
    }

    #[test]
    fn test_evaluation() {
        let board = Board::default();
        let color = Color::White;
        let score = MiniMax::evaluation(&board, color);
        assert_eq!(score, 0);
    }
}
