use crate::board::Board;

const CORNERS: u64 = 0x8100000000000081;
const EDGES: u64 = 0x7e8181818181817e;
const RING: u64 = 0x7e424242427e00; // The outer edge of the board but 1 tile smaller

const CORNER_VAL: i32 = 20;
const EDGE_VAL: i32 = 3;
const RING_VAL: i32 = -5;

fn position_eval(tiles: u64, mask: u64, value: i32) -> i32 {
    (tiles & mask).count_ones() as i32 * value
}

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

pub struct GoodEval {}

impl Evaluator for GoodEval {
    fn eval(&self, board: Board) -> i32 {
        let x = board.get_x();
        let o = board.get_o();

        let score_x = x.count_ones();
        let score_o = o.count_ones();

        let moves_x = board.legal_moves(true);
        let moves_o = board.legal_moves(false);

        // Game over
        if moves_x == 0 && moves_o == 0 {
            if score_x > score_o {
                return i32::MAX - 8;
            } else if score_x < score_o {
                return i32::MIN + 8;
            } else {
                return 0;
            }
        }

        let score_diff = score_x as i32 - score_o as i32;
        let mobility = moves_x.count_ones() as i32 - moves_o.count_ones() as i32;

        let position_x = position_eval(x, CORNERS, CORNER_VAL)
            + position_eval(x, EDGES, EDGE_VAL)
            + position_eval(x, RING, RING_VAL);

        let position_o = position_eval(o, CORNERS, CORNER_VAL)
            + position_eval(o, EDGES, EDGE_VAL)
            + position_eval(o, RING, RING_VAL);

        let position = position_x - position_o;

        score_diff + 2 * mobility + position
    }
}
