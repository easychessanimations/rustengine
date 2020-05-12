use crate::square::*;

/// Bitboard type represents the squares of a 8x8 chess board as bits of an unsigned 64 bit integer
pub type Bitboard = u64;

/// Bitboard trait adds functions to an u64 that allow treating it as a chess engine bitboard
pub trait BitboardTrait {
    /// returns a string that represents the bitboard as pretty print string
    fn pretty_print_string(&self) -> String;
    /// pops a square from the bitboard and returns it
    /// together with a bool indicating whether the pop was succesful
    fn pop(&mut self) -> (Square, bool);
}

/// BitboardTrait adds methods to Bitboard
impl BitboardTrait for Bitboard {
    /// returns a string that represents the bitboard as pretty print string
    fn pretty_print_string(&self) -> String {
        let mut bb = *self;
        let mut buff = "".to_string();
        let mut bits = 0;
        loop {
            if bits % 8 == 0 {
                buff += &format!("{}", NUM_FILES - bits / 8).to_string();
            }
            if bb & (1 << 63) != 0 {
                buff += "1"
            } else {
                buff += "."
            }
            if bits % 8 == 7 {
                buff += "*\n"
            }
            bb = bb << 1;
            bits = bits + 1;
            if bits == 64 {
                break;
            }
        }
        format! {"bitboard {:#016x}\n**********\n{}*abcdefgh*\n", &self, buff}
    }

    /// pops a square from the bitboard and returns it
    /// together with a bool indicating whether the pop was succesful
    fn pop(&mut self) -> (Square, bool) {
        if *self == 0 {
            return (0, false);
        }

        let tzs = self.trailing_zeros() as usize;

        let sq = rank_file(tzs / NUM_FILES, LAST_FILE - (tzs % NUM_FILES));

        *self &= !(1 << tzs);

        return (sq, true);
    }
}
