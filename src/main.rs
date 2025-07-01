use core::panic;
use othello::{
    game::Game,
    player::{Human, PlayerKind, RandomAI},
};
use std::io;
fn main() {
    println!("Choose mode:");
    println!("1 - player vs player");
    println!("2 - player vs random");
    println!("3 - random vs random");

    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);

    let (p1, p2): (PlayerKind, PlayerKind) = match input.trim() {
        "1" => (
            PlayerKind::Human(Human::new('x')),
            PlayerKind::Human(Human::new('o')),
        ),
        "2" => (
            PlayerKind::Human(Human::new('x')),
            PlayerKind::Random(RandomAI::new('o')),
        ),
        "3" => (
            PlayerKind::Random(RandomAI::new('x')),
            PlayerKind::Random(RandomAI::new('o')),
        ),
        _ => panic!("Invalid option"),
    };
    print!("{}[2J", 27 as char); // clear terminal

    let mut game = Game::new(p1, p2);

    game.run();
}
