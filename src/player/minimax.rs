use crate::board::Board;
use crate::eval::Evaluator;
use crate::player::Player;

pub struct Minimax<E: Evaluator> {
    symbol: char,
    depth: u8,
    eval: E,
}

impl<E: Evaluator> Minimax<E> {
    pub const fn new(symbol: char, depth: u8, eval: E) -> Self {
        Self {
            symbol,
            depth,
            eval,
        }
    }

    fn search(&self, board: Board, depth: u8, x_turn: bool) -> (i32, u64) {
        if depth == 0 || board.is_over() {
            return (self.eval.eval(board), 0);
        }

        if board.num_moves(x_turn) == 0 {
            return self.search(board, depth - 1, !x_turn);
        };

        let mut best_score: i32 = if x_turn { i32::MIN } else { i32::MAX };
        let mut best_move: u64 = 0;

        if x_turn {
            for mv in board.moves_iter(x_turn) {
                let mut temp_board = board.clone();
                temp_board.apply_move(mv, x_turn);
                let (eval, _) = self.search(temp_board, depth - 1, !x_turn);
                if eval > best_score {
                    best_move = mv;
                    best_score = eval;
                }
            }
        } else {
            for mv in board.moves_iter(x_turn) {
                let mut temp_board = board.clone();
                temp_board.apply_move(mv, x_turn);
                let (eval, _) = self.search(temp_board, depth - 1, !x_turn);
                if eval < best_score {
                    best_move = mv;
                    best_score = eval;
                }
            }
        }
        (best_score, best_move)
    }
}

impl<E: Evaluator> Player for Minimax<E> {
    fn get_symbol(&self) -> char {
        self.symbol
    }

    fn get_move(&mut self, board: Board) -> Option<u64> {
        if board.num_moves(self.get_symbol() == 'x') == 0 {
            return None;
        }
        let (_, mv) = self.search(board, self.depth, self.get_symbol() == 'x');
        Some(mv)
    }
}
