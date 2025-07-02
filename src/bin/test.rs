use std::fmt;
use std::time::Duration;
use std::time::Instant;

use othello::eval::SimpleEval;
use othello::game::Game;
use othello::game::Outcome;
use othello::player::Minimax;
use othello::player::Player;
use othello::player::RandomAI;
pub struct Stat {
    x_wins: u32,
    draws: u32,
    o_wins: u32,
    iterations: u32,
    duration: Duration,
    description: String,
}

impl Stat {
    pub const fn new(
        x_wins: u32,
        draws: u32,
        o_wins: u32,
        iterations: u32,
        duration: Duration,
        description: String,
    ) -> Self {
        Self {
            x_wins,
            draws,
            o_wins,
            iterations,
            duration,
            description,
        }
    }
}

impl fmt::Display for Stat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:<18} | X {:>4}  O {:>4}  D {:>4} | {:>4} g | {:>.3} s",
            self.description,
            self.x_wins,
            self.o_wins,
            self.draws,
            self.iterations,
            self.duration.as_secs_f64(),
        )
    }
}

pub fn run_bench<P1: Player, P2: Player>(
    p1: P1,
    p2: P2,
    iterations: u32,
    description: impl Into<String>,
) -> Stat {
    let start = Instant::now();

    let mut x_wins: u32 = 0;
    let mut draws: u32 = 0;
    let mut o_wins: u32 = 0;

    let mut game = Game::new(p1, p2);
    for _ in 0..iterations {
        match game.run(false).outcome() {
            Outcome::Xwin => x_wins += 1,
            Outcome::Draw => draws += 1,
            Outcome::OWin => o_wins += 1,
        }
        game.reset();
    }

    let duration = start.elapsed();
    Stat::new(
        x_wins,
        draws,
        o_wins,
        iterations,
        duration,
        description.into(),
    )
}

fn main() {
    let random_o = RandomAI::new('o');
    let random_x = RandomAI::new('x');

    let stat = run_bench(random_x, random_o, 1000, "Random vs random");
    println!("{stat}");

    let mini_x_5 = Minimax::new('x', 5, SimpleEval {});
    let random_o = RandomAI::new('o');
    let stat = run_bench(mini_x_5, random_o, 100, "Mini 5s vs random");
    println!("{stat}");
}
