use crate::board::Board;
use crate::board::MovesIter;
use crate::eval::Evaluator;
use crate::player::Player;
use std::cmp::max;
use std::cmp::min;

pub struct AlphaBeta<E: Evaluator> {
    symbol: char,
    depth: u8,
    eval: E,
}

impl<E: Evaluator> AlphaBeta<E> {
    pub const fn new(symbol: char, depth: u8, eval: E) -> Self {
        Self {
            symbol,
            depth,
            eval,
        }
    }

    fn search(
        &self,
        board: Board,
        depth: u8,
        x_turn: bool,
        mut alpha: i32,
        mut beta: i32,
    ) -> (i32, u64) {
        if depth == 0 || board.is_over() {
            return (self.eval.eval(board), 0);
        }

        let moves = board.legal_moves(x_turn);
        if moves == 0 {
            return self.search(board, depth - 1, !x_turn, alpha, beta);
        };

        let mut best_score: i32 = if x_turn { i32::MIN } else { i32::MAX };
        let mut best_move: u64 = 0;

        if x_turn {
            for mv in MovesIter::new(moves) {
                let mut temp_board = board.clone();
                temp_board.apply_move(mv, x_turn);
                let (eval, _) = self.search(temp_board, depth - 1, !x_turn, alpha, beta);
                alpha = max(alpha, eval);
                if eval > best_score {
                    best_move = mv;
                    best_score = eval;
                }
                if beta <= alpha {
                    break;
                }
            }
        } else {
            for mv in MovesIter::new(moves) {
                let mut temp_board = board.clone();
                temp_board.apply_move(mv, x_turn);
                let (eval, _) = self.search(temp_board, depth - 1, !x_turn, alpha, beta);
                beta = min(beta, eval);
                if eval < best_score {
                    best_move = mv;
                    best_score = eval;
                }
                if beta <= alpha {
                    break;
                }
            }
        }
        (best_score, best_move)
    }
}

impl<E: Evaluator> Player for AlphaBeta<E> {
    fn get_symbol(&self) -> char {
        self.symbol
    }

    fn get_move(&mut self, board: Board) -> Option<u64> {
        if board.num_moves(self.get_symbol() == 'x') == 0 {
            return None;
        }
        let (_, mv) = self.search(
            board,
            self.depth,
            self.get_symbol() == 'x',
            i32::MIN,
            i32::MAX,
        );
        Some(mv)
    }
}
