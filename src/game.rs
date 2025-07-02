use crate::board::Board;
use crate::player::Player;

#[derive(Clone)]
pub struct Game<P1: Player, P2: Player> {
    board: Board,
    p1: P1,
    p2: P2,
    x_turn: bool,
}

impl<P1: Player, P2: Player> Game<P1, P2> {
    pub fn new(p1: P1, p2: P2) -> Self {
        Self {
            board: Board::default(),
            p1, // x
            p2, // o
            x_turn: true,
        }
    }

    pub fn run(&mut self, print: bool) -> Score {
        loop {
            if print {
                println!();
                self.board.print(self.x_turn);
            }

            if self.x_turn && print {
                println!("X to move");
            } else if print {
                println!("O to move");
            }

            let mv = if self.x_turn {
                self.p1.get_move(self.board.clone())
            } else {
                self.p2.get_move(self.board.clone())
            };

            match mv {
                Some(mv) => self.board.apply_move(mv, self.x_turn),
                None => {
                    if print {
                        println!("No moves available - skipping turn")
                    }
                }
            }

            self.x_turn = !self.x_turn;

            if self.board.is_over() {
                if print {
                    self.board.print(self.x_turn);
                }

                return Score::new(
                    self.board.get_x().count_ones(),
                    self.board.get_o().count_ones(),
                );
            }
        }
    }
    pub fn reset(&mut self) {
        self.board = Board::new();
    }
}

pub struct Score {
    x: u32,
    o: u32,
}

impl Score {
    pub const fn new(x: u32, o: u32) -> Self {
        Self { x, o }
    }

    pub const fn x(&self) -> u32 {
        self.x
    }

    pub const fn o(&self) -> u32 {
        self.o
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
