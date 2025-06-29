use std::io;

use crate::board::Board;

pub trait Player {
    fn new(symbol: char) -> Self;
    fn get_move(&self, board: &Board) -> u64;
    fn get_symbol(&self) -> char;
}

struct Human {
    symbol: char,
}

impl Human {
    pub fn parse_input(input: &str) -> Option<(u8, u8)> {
        let mut chars = input.chars();
        let file = chars.next()?;
        let rank = chars.next()?;
        if chars.next().is_some() {
            return None;
        }
        Some((rank as u8 - b'1', file as u8 - b'a'))
    }
}

impl Player for Human {
    fn new(symbol: char) -> Human {
        Human { symbol }
    }

    fn get_symbol(&self) -> char {
        self.symbol
    }

    fn get_move(&self, board: &Board) -> u64 {
        let mut input = String::new();
        println!("Enter move:");
        loop {
            let _ = io::stdin().read_line(&mut input);
            Human::parse_input(&input).and_then(|(row, col)| {
                let mv = Board::get_move(row, col);
                if board.is_legal(mv, self.get_symbol() == 'x') {
                    Some(mv)
                } else {
                    print!("Illegal move.");
                    None
                }
            });
        }
    }
}
