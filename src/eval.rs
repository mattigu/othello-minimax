use crate::board::Board;

pub trait Evaluator {
    fn eval(&self, board: Board) -> i32;
}

pub struct SimpleEval {}

impl Evaluator for SimpleEval {
    fn eval(&self, board: Board) -> i32 {
        let score_x = board.get_x().count_ones();
        let score_o = board.get_o().count_ones();
        score_x as i32 - score_o as i32
    }
}
