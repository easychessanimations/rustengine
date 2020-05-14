use crate::constants::*;
use crate::square::*;

/// Bitboard type represents the squares of a 8x8 chess board as bits of an unsigned 64 bit integer
pub type Bitboard = u64;

/// Bitboard trait adds functions to an u64 that allow treating it as a chess engine bitboard
pub trait BitboardTrait {
    /// returns a string that represents the bitboard as pretty print string
    fn pretty_print_string(&self) -> String;
    /// pops a bitboard with only one bit set from the bitboard and returns it
    /// together with a bool indicating whether the pop was succesful
    fn pop_bitboard(&mut self) -> (Bitboard, bool);
    /// pops a square from the bitboard and returns it
    /// together with a bool indicating whether the pop was succesful
    fn pop_square(&mut self) -> (Square, bool);
    /// returns the number of ways the 1 bits in the bitboard can be set to either 1 or 0
    fn variation_count(self) -> usize;
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

    /// pops a bitboard with only one bit set from the bitboard and returns it
    /// together with a bool indicating whether the pop was succesful
    fn pop_bitboard(&mut self) -> (Bitboard, bool) {
        if *self == 0 {
            return (0, false);
        }

        let bb = 1 << (self.trailing_zeros() as usize);

        *self &= !bb;

        return (bb, true);
    }

    /// pops a square from the bitboard and returns it
    /// together with a bool indicating whether the pop was succesful
    fn pop_square(&mut self) -> (Square, bool) {
        let (bb, ok) = self.pop_bitboard();

        if ok {
            let tzs = bb.trailing_zeros() as usize;

            (
                rank_file(tzs / NUM_FILES, LAST_FILE - (tzs % NUM_FILES)),
                true,
            )
        } else {
            (0, false)
        }
    }

    /// returns the number of ways the 1 bits in the bitboard can be set to either 1 or 0
    fn variation_count(self) -> usize {
        1 << self.count_ones()
    }
}
