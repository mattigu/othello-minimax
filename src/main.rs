use core::panic;
use othello::{
    eval::{GoodEval, SimpleEval},
    game::{Game, Outcome},
    player::{Human, Negamax, Player, PlayerKind, RandomAI},
};
use std::io;

fn run_game<P1: Player, P2: Player>(p1: P1, p2: P2) {
    let mut game = Game::new(p1, p2);
    let score = game.run(true);
    match score.outcome() {
        Outcome::Xwin => println!("X won"),
        Outcome::Draw => println!("Draw"),
        Outcome::OWin => println!("O won"),
    }
    println!("x - {} to o - {} tiles", score.x(), score.o());
}

fn main() {
    println!("Choose mode:");
    println!("1 - player vs player");
    println!("2 - player vs random");
    println!("3 - random vs random");
    println!("4 - player vs best algorithm");

    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);

    type PKSim = PlayerKind<SimpleEval>;
    type PKBest = PlayerKind<GoodEval>;

    match input.trim() {
        "1" => run_game(PKSim::Human(Human::new('x')), PKSim::Human(Human::new('o'))),

        "2" => run_game(
            PKSim::Human(Human::new('x')),
            PKSim::Random(RandomAI::new('o')),
        ),
        "3" => run_game(
            PKSim::Random(RandomAI::new('x')),
            PKSim::Random(RandomAI::new('o')),
        ),
        "4" => run_game(
            PKSim::Human(Human::new('x')),
            PKBest::Negamax(Negamax::new('o', 10, GoodEval {})),
        ),

        _ => panic!("Invalid option"),
    };
    print!("{}[2J", 27 as char); // clear terminal
}
