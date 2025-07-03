use crate::board::Board;
use crate::player::Player;
use std::io;

pub struct Human {
    symbol: char,
}

impl Human {
    pub const fn new(symbol: char) -> Self {
        Self { symbol }
    }

    pub fn parse_input(input: &str) -> Option<u64> {
        let mut chars = input.chars();
        let file = chars.next()?;
        let rank = chars.next()?;
        if chars.next().is_some() {
            return None;
        }
        let mv = Board::get_move(b'8' - rank as u8, file as u8 - b'a');
        Some(mv)
    }
}

impl Player for Human {
    fn get_symbol(&self) -> char {
        self.symbol
    }

    fn get_move(&mut self, board: Board) -> Option<u64> {
        if board.num_moves(self.get_symbol() == 'x') == 0 {
            return None;
        }
        let mut input = String::new();
        println!("Enter move");
        loop {
            input.clear();
            let _ = io::stdin().read_line(&mut input);

            if let Some(mv) = Human::parse_input(input.trim())
                && board.is_legal(mv, self.get_symbol() == 'x')
            {
                return Some(mv);
            }
            println!("Illegal move.");
        }
    }
}
