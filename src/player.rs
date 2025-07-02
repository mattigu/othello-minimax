use crate::board::Board;
use crate::eval::Evaluator;
use rand::random_range;
use std::io;

pub trait Player {
    fn get_move(&self, board: Board) -> Option<u64>;
    fn get_symbol(&self) -> char;
}

pub enum PlayerKind<E: Evaluator> {
    Human(Human),
    Random(RandomAI),
    Minimax(Minimax<E>),
}

impl<E: Evaluator> Player for PlayerKind<E> {
    fn get_symbol(&self) -> char {
        match self {
            PlayerKind::Human(p) => p.get_symbol(),
            PlayerKind::Random(p) => p.get_symbol(),
            PlayerKind::Minimax(p) => p.get_symbol(),
        }
    }

    fn get_move(&self, board: Board) -> Option<u64> {
        match self {
            PlayerKind::Human(p) => p.get_move(board),
            PlayerKind::Random(p) => p.get_move(board),
            PlayerKind::Minimax(p) => p.get_move(board),
        }
    }
}

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

    fn get_move(&self, board: Board) -> Option<u64> {
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

pub struct RandomAI {
    symbol: char,
}

impl RandomAI {
    pub const fn new(symbol: char) -> Self {
        Self { symbol }
    }
}

impl Player for RandomAI {
    fn get_symbol(&self) -> char {
        self.symbol
    }

    fn get_move(&self, board: Board) -> Option<u64> {
        let num_moves = board.num_moves(self.get_symbol() == 'x');
        if num_moves == 0 {
            return None;
        }
        let num = random_range(0..num_moves) as usize;
        let mv = board.moves_iter(self.get_symbol() == 'x').nth(num).unwrap();
        Some(mv)
    }
}

pub struct Minimax<E: Evaluator> {
    symbol: char,
    depth: u8,
    eval: E,
}

impl<E: Evaluator> Minimax<E> {
    pub const fn new(symbol: char, depth: u8, eval: E) -> Self {
        Self {
            symbol,
            depth,
            eval,
        }
    }

    fn search(&self, board: Board, depth: u8, x_turn: bool) -> (i32, u64) {
        if depth == 0 || board.is_over() {
            return (self.eval.eval(board), 0);
        }

        if board.num_moves(x_turn) == 0 {
            return self.search(board, depth - 1, !x_turn);
        };

        let mut best_score: i32 = if x_turn { i32::MIN } else { i32::MAX };
        let mut best_move: u64 = 0;

        if x_turn {
            for mv in board.moves_iter(x_turn) {
                let mut temp_board = board.clone();
                temp_board.apply_move(mv, x_turn);
                let (eval, _) = self.search(temp_board, depth - 1, !x_turn);
                if eval > best_score {
                    best_move = mv;
                    best_score = eval;
                }
            }
        } else {
            for mv in board.moves_iter(x_turn) {
                let mut temp_board = board.clone();
                temp_board.apply_move(mv, x_turn);
                let (eval, _) = self.search(temp_board, depth - 1, !x_turn);
                if eval < best_score {
                    best_move = mv;
                    best_score = eval;
                }
            }
        }
        (best_score, best_move)
    }
}

impl<E: Evaluator> Player for Minimax<E> {
    fn get_symbol(&self) -> char {
        self.symbol
    }

    fn get_move(&self, board: Board) -> Option<u64> {
        if board.num_moves(self.get_symbol() == 'x') == 0 {
            return None;
        }
        let (_, mv) = self.search(board, self.depth, self.get_symbol() == 'x');
        Some(mv)
    }
}
