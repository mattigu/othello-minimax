use crate::board::Board;
use crate::eval::Evaluator;
use crate::player::Player;
use std::cmp::max;

pub struct Negamax<E: Evaluator> {
    symbol: char,
    depth: u8,
    eval: E,
}

impl<E: Evaluator> Negamax<E> {
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
        color: i8,
        mut alpha: i32,
        beta: i32,
    ) -> (i32, u64) {
        if depth == 0 || board.is_over() {
            return (color as i32 * self.eval.eval(board), 0);
        }

        if board.num_moves(x_turn) == 0 {
            let (eval, mv) = self.search(board, depth - 1, !x_turn, -color, -beta, -alpha);
            return (-eval, mv);
        };

        let mut value = i32::MIN + 1;
        let mut best_move = 0;

        for mv in board.moves_iter(x_turn) {
            let mut temp_board = board.clone();
            temp_board.apply_move(mv, x_turn);
            let (eval, _) = self.search(temp_board, depth - 1, !x_turn, -color, -beta, -alpha);
            if -eval > value {
                value = -eval;
                best_move = mv;
            }
            alpha = max(alpha, value);
            if alpha >= beta {
                break;
            }
        }

        (value, best_move)
    }
}

impl<E: Evaluator> Player for Negamax<E> {
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
            if self.get_symbol() == 'x' { 1 } else { -1 },
            i32::MIN + 1,
            i32::MAX - 1,
        );
        Some(mv)
    }
}
