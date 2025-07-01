use crate::board::Board;
use crate::player::{Player, PlayerKind};
pub struct Game {
    board: Board,
    p1: PlayerKind,
    p2: PlayerKind,
    x_turn: bool,
}

impl Game {
    pub fn new(p1: PlayerKind, p2: PlayerKind) -> Self {
        Self {
            board: Board::default(),
            p1, // x
            p2, // o
            x_turn: true,
        }
    }

    pub fn run(&mut self) -> Score {
        loop {
            println!();
            self.board.print(self.x_turn);
            let mv = if self.x_turn {
                println!("X to move:");
                self.p1.get_move(self.board.clone())
            } else {
                println!("O to move");
                self.p2.get_move(self.board.clone())
            };

            match mv {
                Some(mv) => self.board.apply_move(mv, self.x_turn),
                None => println!("No moves available - skipping turn"),
            }

            self.x_turn = !self.x_turn;

            if self.is_over() {
                return Score::new(
                    self.board.get_x().count_ones(),
                    self.board.get_o().count_ones(),
                );
            }
        }
    }

    fn is_over(&self) -> bool {
        self.board.num_moves(true) == 0 && self.board.num_moves(false) == 0
    }
}

pub struct Score {
    x: u32,
    o: u32,
}

impl Score {
    pub fn new(x: u32, o: u32) -> Self {
        Self { x, o }
    }

    pub const fn outcome(&self) -> Outcome {
        if self.x > self.o {
            Outcome::Xwin
        } else if self.o > self.x {
            Outcome::OWin
        } else {
            Outcome::Draw
        }
    }
}

pub enum Outcome {
    Xwin,
    OWin,
    Draw,
}
