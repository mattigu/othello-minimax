use std::fmt;
pub struct Board {
    x: u64,
    o: u64,
}

impl Board {
    pub fn new() -> Self {
        Self {
            x: (1 << 27) + (1 << 36),
            o: (1 << 28) + (1 << 35),
        }
    }

    const fn get_idx(row: u8, col: u8) -> u8 {
        row * 8 + col
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

    const fn trail_ones(n: u32) -> u64 {
        (1 << n) - 1
    }

    fn apply_horizontal(xo: u64, tiles: u64, bit_idx: u32) -> u64 {
        let t = tiles >> bit_idx;
        let t2 = xo >> bit_idx;
        let mut mask = 0;
        if t.trailing_zeros() < t2.trailing_ones() && t2.trailing_ones() < 8 - (bit_idx % 8) {
            mask |= Board::trail_ones(t.trailing_zeros()) << bit_idx;
        };
        mask
    }

    pub fn apply_move(&mut self, row: u8, col: u8, x: bool) {
        let mut mask = 0u64;
        let bit_idx = Board::get_idx(row, col) as u32;
        let tiles = if x { self.x } else { self.o };
        let xo = self.x | self.o | (1u64 << bit_idx);

        // Horizontal right
        mask |= Board::apply_horizontal(xo, tiles, bit_idx);

        // Horizontal left
        let rev_tiles = tiles.reverse_bits();
        let rev_xo = xo.reverse_bits();

        mask |= Board::apply_horizontal(rev_xo, rev_tiles, 63 - bit_idx).reverse_bits();

        // Vertical down
        mask |= Board::transpose8x8(Board::apply_horizontal(
            Board::transpose8x8(xo),
            Board::transpose8x8(tiles),
            Board::get_idx(col, row) as u32,
        ));

        // Vertical up
        mask |= Board::transpose8x8(Board::apply_horizontal(
            Board::transpose8x8(rev_xo),
            Board::transpose8x8(rev_tiles),
            Board::get_idx(7 - col, 7 - row) as u32,
        ))
        .reverse_bits();

        self.flip_tiles(mask, x);
    }

    fn flip_tiles(&mut self, mask: u64, x: bool) {
        if x {
            self.x |= mask;
            self.o &= !mask;
        } else {
            self.x &= !mask;
            self.o |= mask;
        }
    }

    fn transpose8x8(mut x: u64) -> u64 {
        let t = (x ^ (x >> 7)) & 0x00AA_00AA_00AA_00AA;
        x ^= t ^ (t << 7);
        let t = (x ^ (x >> 14)) & 0x0000_CCCC_0000_CCCC;
        x ^= t ^ (t << 14);
        let t = (x ^ (x >> 28)) & 0x0000_0000_F0F0_F0F0;
        x ^= t ^ (t << 28);
        x
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
            for col in 0..8 {
                write!(f, " {} ", self.at(row, col))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// Bit magic
