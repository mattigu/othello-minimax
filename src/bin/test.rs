use std::fmt;
use std::time::Duration;
use std::time::Instant;

use othello::eval::GoodEval;
use othello::eval::SimpleEval;
use othello::game::Game;
use othello::game::Outcome;
use othello::player::AlphaBeta;
use othello::player::Minimax;
use othello::player::Mixed;
use othello::player::Negamax;
use othello::player::Player;
use othello::player::RandomAI;
use othello::utils::ansi_for;
use othello::utils::color;
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

    fn print_header() {
        println!(
            "{:<22} | {:^28} | {:^6} | {:^8} | {:^5}",
            "Description", "results", "runs", "total", "avg"
        );
    }

    fn format_time(&self, duration: Duration) -> String {
        let time = duration.as_nanos();
        if time < 1000 {
            format!("{time} ns")
        } else if time < 1_000_000 {
            format!("{:.1} μs", time as f64 / 1_000.0)
        } else if time < 1_000_000_000 {
            format!("{:.1} ms", time as f64 / 1_000_000.0)
        } else {
            format!("{:.1} s", duration.as_secs_f64())
        }
    }
}

impl fmt::Display for Stat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:<22} | {} {:>6}  {} {:>6}  D {:>6} | {:>6} | {:>8} | {:>5}",
            self.description,
            color("X", ansi_for('x')),
            self.x_wins,
            color("O", ansi_for('o')),
            self.o_wins,
            self.draws,
            self.iterations,
            self.format_time(self.duration),
            self.format_time(self.duration / self.iterations),
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
    // naming scheme - algorithm_symbol_depth_evaluator
    const SEED: u64 = 0x_A142_3141_A150_4411;
    Stat::print_header();

    let random_o = RandomAI::with_seed('o', SEED);
    let random_x = RandomAI::with_seed('x', SEED);
    let stat = run_bench(random_x, random_o, 100000, "Random vs random");
    println!("{stat}");

    let mini_x_s_4 = Minimax::new('x', 4, SimpleEval {});
    let random_o = RandomAI::with_seed('o', SEED);
    let stat = run_bench(mini_x_s_4, random_o, 200, "Mini 4s vs random");
    println!("{stat}");

    let alpha_x_s_6 = AlphaBeta::new('x', 6, SimpleEval {});
    let random_o = RandomAI::with_seed('o', SEED);
    let stat = run_bench(alpha_x_s_6, random_o, 100, "Alpha 6s vs random");
    println!("{stat}");

    let nega_x_s_6 = Negamax::new('x', 6, SimpleEval {});
    let random_o = RandomAI::with_seed('o', SEED);
    let stat = run_bench(nega_x_s_6, random_o, 100, "Nega 6s vs random");
    println!("{stat}");

    let nega_x_b_5 = Negamax::new('x', 5, GoodEval {});
    let random_o = RandomAI::with_seed('o', SEED);
    let stat = run_bench(nega_x_b_5, random_o, 100, "Nega 5B vs random");
    println!("{stat}");

    let nega_x_b_5 = Negamax::new('x', 3, GoodEval {});
    let mixed_50_nega_s_5 = Mixed::new(Negamax::new('o', 8, SimpleEval {}), 'o', SEED, 0.2);
    let stat = run_bench(nega_x_b_5, mixed_50_nega_s_5, 50, "Nega 5B vs 20% nega_9S");
    println!("{stat}");
}
