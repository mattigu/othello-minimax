use crate::utils::ansi_for;
use crate::utils::color;
use std::fmt;

const NOT_FILE_A: u64 = 0xFEFE_FEFE_FEFE_FEFE;
const NOT_FILE_H: u64 = 0x7F7F_7F7F_7F7F_7F7F;
const NOT_RANK_1: u64 = 0x00FF_FFFF_FFFF_FFFF;
const NOT_RANK_8: u64 = 0xFFFF_FFFF_FFFF_FF00;

// Functions to move in a direction
fn e(x: u64) -> u64 {
    (x << 1) & NOT_FILE_A
}
fn w(x: u64) -> u64 {
    (x >> 1) & NOT_FILE_H
}
fn s(x: u64) -> u64 {
    (x << 8) & NOT_RANK_8
}
fn n(x: u64) -> u64 {
    (x >> 8) & NOT_RANK_1
}
fn ne(x: u64) -> u64 {
    (x >> 7) & NOT_RANK_1 & NOT_FILE_A
}
fn se(x: u64) -> u64 {
    (x << 9) & NOT_RANK_8 & NOT_FILE_A
}
fn nw(x: u64) -> u64 {
    (x >> 9) & NOT_RANK_1 & NOT_FILE_H
}
fn sw(x: u64) -> u64 {
    (x << 7) & NOT_RANK_8 & NOT_FILE_H
}
#[derive(Clone)]
pub struct Board {
    x: u64,
    o: u64,
}
// a8 is the LSB in the bitboards.

impl Board {
    pub fn new() -> Self {
        Self {
            x: (1 << 27) + (1 << 36),
            o: (1 << 28) + (1 << 35),
        }
    }
    pub const fn get_x(&self) -> u64 {
        self.x
    }

    pub const fn get_o(&self) -> u64 {
        self.o
    }

    const fn get_idx(row: u8, col: u8) -> u8 {
        row * 8 + col
    }

    pub const fn get_move(row: u8, col: u8) -> u64 {
        1 << Board::get_idx(row, col)
    }

    const fn get_me_opp(&self, x_turn: bool) -> (u64, u64) {
        if x_turn {
            (self.x, self.o)
        } else {
            (self.o, self.x)
        }
    }

    const fn at(&self, row: u8, col: u8) -> char {
        let idx = 1u64 << Board::get_idx(row, col);
        let x_val = self.x & idx;
        let o_val = self.o & idx;
        if x_val != 0 {
            'x'
        } else if o_val != 0 {
            'o'
        } else {
            '·'
        }
    }

    pub fn apply_move(&mut self, mv: u64, x_turn: bool) {
        let mut mask = 0u64;
        let (me, opp) = self.get_me_opp(x_turn);

        mask |= Board::calc_flips(mv, e, me, opp)
            | Board::calc_flips(mv, w, me, opp)
            | Board::calc_flips(mv, n, me, opp)
            | Board::calc_flips(mv, s, me, opp)
            | Board::calc_flips(mv, se, me, opp)
            | Board::calc_flips(mv, sw, me, opp)
            | Board::calc_flips(mv, nw, me, opp)
            | Board::calc_flips(mv, ne, me, opp)
            | mv;

        self.flip_tiles(mask, x_turn);
    }

    fn calc_flips(mv: u64, ray: fn(u64) -> u64, me: u64, opp: u64) -> u64 {
        let mut run = ray(mv) & opp;
        for _ in 0..5 {
            run |= ray(run) & opp
        }
        if (ray(run) & me) != 0 { run } else { 0 }
    }

    fn flip_tiles(&mut self, mask: u64, x_turn: bool) {
        if x_turn {
            self.x |= mask;
            self.o &= !mask;
        } else {
            self.x &= !mask;
            self.o |= mask;
        }
    }

    fn moves_dir(ray: fn(u64) -> u64, me: u64, opp: u64, empty: u64) -> u64 {
        let mut t = ray(me) & opp;
        for _ in 0..5 {
            t |= ray(t) & opp;
        }
        ray(t) & empty
    }

    pub fn legal_moves(&self, x_turn: bool) -> u64 {
        let (me, opp) = self.get_me_opp(x_turn);
        let empty = !(me | opp);

        Board::moves_dir(e, me, opp, empty)
            | Board::moves_dir(w, me, opp, empty)
            | Board::moves_dir(n, me, opp, empty)
            | Board::moves_dir(s, me, opp, empty)
            | Board::moves_dir(se, me, opp, empty)
            | Board::moves_dir(sw, me, opp, empty)
            | Board::moves_dir(nw, me, opp, empty)
            | Board::moves_dir(ne, me, opp, empty)
    }

    pub fn is_legal(&self, mv: u64, x_turn: bool) -> bool {
        self.legal_moves(x_turn) & mv != 0
    }

    pub fn num_moves(&self, x_turn: bool) -> u32 {
        self.legal_moves(x_turn).count_ones()
    }

    // Separate print to color move suggestions dependent on turn.
    pub fn print(&self, x_turn: bool) {
        let side = if x_turn { 'x' } else { 'o' };
        for row in 0..8 {
            print!("{}", 8 - row);
            for col in 0..8 {
                let idx = Board::get_idx(row, col);
                let mask = 1u64 << idx;
                let symbol = self.at(row, col);

                if symbol == '·' && self.is_legal(mask, x_turn) {
                    print!(" {} ", color(&symbol.to_string(), ansi_for(side)))
                } else {
                    print!(" {} ", color(&symbol.to_string(), ansi_for(symbol)));
                }
            }
            println!();
        }
        println!("  a  b  c  d  e  f  g  h  ");
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..8 {
            write!(f, "{}", 8 - row)?;
            for col in 0..8 {
                write!(f, " {} ", self.at(row, col))?;
            }
            writeln!(f)?;
        }
        writeln!(f, "  a  b  c  d  e  f  g  h  ")?;
        Ok(())
    }
}
