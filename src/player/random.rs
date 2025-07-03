use crate::board::Board;
use crate::player::Player;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::SmallRng;

pub struct RandomAI {
    symbol: char,
    rng: SmallRng,
}

impl RandomAI {
    pub fn new(symbol: char) -> Self {
        Self {
            symbol,
            rng: SmallRng::from_os_rng(),
        }
    }

    pub fn with_seed(symbol: char, seed: u64) -> Self {
        Self {
            symbol,
            rng: SmallRng::seed_from_u64(seed),
        }
    }
}

impl Player for RandomAI {
    fn get_symbol(&self) -> char {
        self.symbol
    }

    fn get_move(&mut self, board: Board) -> Option<u64> {
        let num_moves = board.num_moves(self.get_symbol() == 'x');
        if num_moves == 0 {
            return None;
        }
        let num = self.rng.random_range(0..num_moves) as usize;
        let mv = board.moves_iter(self.get_symbol() == 'x').nth(num).unwrap();
        Some(mv)
    }
}
