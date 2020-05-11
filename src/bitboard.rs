/// Bitboard type represents the squares of a 8x8 chess board as bits of an unsigned 64 bit integer
pub type Bitboard = u64;

/// Bitboard trait adds functions to an u64 that allow treating it as a chess engine bitboard
pub trait BitboardTrait {
    /// returns a string that represents the bitboard as pretty print string
    fn pretty_print_string(&self) -> String;
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
                buff += "*"
            }
            if bb & 1 == 1 {
                buff += "1"
            } else {
                buff += "0"
            }
            if bits % 8 == 7 {
                buff += "*\n"
            }
            bb = bb >> 1;
            bits = bits + 1;
            if bits == 64 {
                break;
            }
        }
        format! {"bitboard {:#016x}\n**********\n{}**********\n", &self, buff}
    }
}
