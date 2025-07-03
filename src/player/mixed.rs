use crate::player::{Player, RandomAI};
use rand::{Rng, SeedableRng, rngs::SmallRng};

// Since the best algorithm wins against random moves 100% of the time
// there needs to be something non deterministic to benchmark against
pub struct Mixed<P1: Player> {
    p1: P1,
    p2: RandomAI,
    symbol: char,
    rand_chance: f64,
    rng: SmallRng,
}

impl<P1: Player> Mixed<P1> {
    pub fn new(p1: P1, symbol: char, seed: u64, rand_chance: f64) -> Self {
        Self {
            p1,
            p2: RandomAI::with_seed(symbol, seed),
            symbol,
            rand_chance,
            rng: SmallRng::seed_from_u64(seed),
        }
    }
}

impl<P1: Player> Player for Mixed<P1> {
    fn get_symbol(&self) -> char {
        self.symbol
    }

    fn get_move(&mut self, board: crate::board::Board) -> Option<u64> {
        let random: f64 = self.rng.random();
        if random < self.rand_chance {
            self.p2.get_move(board)
        } else {
            self.p1.get_move(board)
        }
    }
}
