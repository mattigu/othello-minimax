pub mod alphabeta;
pub mod human;
pub mod minimax;
pub mod negamax;
pub mod random;

use crate::board::Board;
use crate::eval::Evaluator;

pub use alphabeta::AlphaBeta;
pub use human::Human;
pub use minimax::Minimax;
pub use negamax::Negamax;
pub use random::RandomAI;

pub trait Player {
    fn get_move(&mut self, board: Board) -> Option<u64>;
    fn get_symbol(&self) -> char;
}

pub enum PlayerKind<E: Evaluator> {
    Human(Human),
    Random(RandomAI),
    Minimax(Minimax<E>),
    AlphaBeta(AlphaBeta<E>),
    Negamax(Negamax<E>),
}

impl<E: Evaluator> Player for PlayerKind<E> {
    fn get_symbol(&self) -> char {
        match self {
            PlayerKind::Human(p) => p.get_symbol(),
            PlayerKind::Random(p) => p.get_symbol(),
            PlayerKind::Minimax(p) => p.get_symbol(),
            PlayerKind::AlphaBeta(p) => p.get_symbol(),
            PlayerKind::Negamax(p) => p.get_symbol(),
        }
    }

    fn get_move(&mut self, board: Board) -> Option<u64> {
        match self {
            PlayerKind::Human(p) => p.get_move(board),
            PlayerKind::Random(p) => p.get_move(board),
            PlayerKind::Minimax(p) => p.get_move(board),
            PlayerKind::AlphaBeta(p) => p.get_move(board),
            PlayerKind::Negamax(p) => p.get_move(board),
        }
    }
}
