use crate::bitboard::*;

use once_cell::sync::Lazy;

extern crate rand;

use rand::Rng;

use std::io::{BufWriter, Write};

/// Rank type represents the rank of a square as an unsigned int
pub type Rank = usize;
/// Rank type represents the file of a square as an unsigned int
pub type File = usize;
/// Square type represents a square of a chess board as an unsigned int
pub type Square = usize;

/// NUM_RANKS tells the number of ranks of a chess board
pub const NUM_RANKS: usize = 8;
/// LAST_RANK tells the last rank of a chess board
pub const LAST_RANK: Rank = NUM_RANKS - 1;
/// ONE_BEFORE_LAST_RANK tells the rank before last rank
pub const ONE_BEFORE_LAST_RANK: Rank = LAST_RANK - 1;
/// TWO_BEFORE_LAST_RANK tells the rank two before last rank
pub const TWO_BEFORE_LAST_RANK: Rank = LAST_RANK - 2;
/// NUM_RANKS tells the number of files of a chess board
pub const NUM_FILES: usize = 8;
/// LAST_FILE tells the last file of a chess board
pub const LAST_FILE: File = NUM_FILES - 1;
/// ONE_BEFORE_LAST_FILE tells the file before last file
pub const ONE_BEFORE_LAST_FILE: File = LAST_FILE - 1;
/// TWO_BEFORE_LAST_FILE tells the file two before last file
pub const TWO_BEFORE_LAST_FILE: File = LAST_FILE - 2;
/// BOARD_AREA tells the size of the board in squares
pub const BOARD_AREA: usize = NUM_RANKS * NUM_FILES;

/// Delta enum lists the possible deltas of chess pieces
pub enum Delta {
    N,
    NE,
    NNE,
    NEE,
    E,
    SE,
    SEE,
    SSE,
    S,
    SW,
    SSW,
    SWW,
    W,
    NW,
    NWW,
    NNW,
}

/// DeltaBuffer trait adds methods to various size buffers that hold deltas
pub trait DeltaBuffer {
    /// len tells the length of the delta buffer
    fn len(&self) -> usize;
    /// get gets a delta by index from the delta buffer
    fn get(&self, i: usize) -> &Delta;
}

// DeltaBuffer trait adds methods to 4 buffer that hold deltas
impl DeltaBuffer for [Delta; 4] {
    /// len tells the length of the delta buffer
    fn len(&self) -> usize {
        4
    }
    /// get gets a delta by index from the delta buffer
    fn get(&self, i: usize) -> &Delta {
        return &self[i];
    }
}

// DeltaBuffer trait adds methods to 8 buffer that hold deltas
impl DeltaBuffer for [Delta; 8] {
    /// len tells the length of the delta buffer
    fn len(&self) -> usize {
        8
    }
    /// get gets a delta by index from the delta buffer
    fn get(&self, i: usize) -> &Delta {
        return &self[i];
    }
}

/// KNIGHT_DELTAS lists the possible deltas of a knight
pub const KNIGHT_DELTAS: [Delta; 8] = [
    Delta::NNE,
    Delta::NEE,
    Delta::SEE,
    Delta::SSE,
    Delta::SSW,
    Delta::SWW,
    Delta::NWW,
    Delta::NNW,
];

/// BISHOP_DELTAS lists the possible deltas of a bishop
pub const BISHOP_DELTAS: [Delta; 4] = [Delta::NE, Delta::SE, Delta::SW, Delta::NW];

/// ROOK_DELTAS lists the possible deltas of a rook
pub const ROOK_DELTAS: [Delta; 4] = [Delta::N, Delta::E, Delta::S, Delta::W];

/// QUEEN_DELTAS lists the possible deltas of a queen
pub const QUEEN_DELTAS: [Delta; 8] = [
    Delta::N,
    Delta::NE,
    Delta::E,
    Delta::SE,
    Delta::S,
    Delta::SW,
    Delta::W,
    Delta::NW,
];

/// RANK_SHIFT tells the position of the rank in bits within a square
pub const RANK_SHIFT: usize = 3;
/// FILE_MASK is a mask that has bits set that represent the file within a square
pub const FILE_MASK: usize = (1 << RANK_SHIFT) - 1;

/// RANK_1 represents rank '1' of a chess board
pub const RANK_1: Rank = 0;
/// RANK_2 represents rank '2' of a chess board
pub const RANK_2: Rank = 1;
/// RANK_3 represents rank '3' of a chess board
pub const RANK_3: Rank = 2;
/// RANK_4 represents rank '4' of a chess board
pub const RANK_4: Rank = 3;
/// RANK_5 represents rank '5' of a chess board
pub const RANK_5: Rank = 4;
/// RANK_6 represents rank '6' of a chess board
pub const RANK_6: Rank = 5;
/// RANK_7 represents rank '7' of a chess board
pub const RANK_7: Rank = 6;
/// RANK_8 represents rank '8' of a chess board
pub const RANK_8: Rank = 7;

/// FILE_A represents file 'a' of a chess board
pub const FILE_A: File = 0;
/// FILE_B represents file 'b' of a chess board
pub const FILE_B: File = 1;
/// FILE_C represents file 'c' of a chess board
pub const FILE_C: File = 2;
/// FILE_D represents file 'd' of a chess board
pub const FILE_D: File = 3;
/// FILE_E represents file 'e' of a chess board
pub const FILE_E: File = 4;
/// FILE_F represents file 'f' of a chess board
pub const FILE_F: File = 5;
/// FILE_G represents file 'g' of a chess board
pub const FILE_G: File = 6;
/// FILE_H represents file 'h' of a chess board
pub const FILE_H: File = 7;

/// SQUARE_A1 represents square 'a1' of a chess board
pub const SQUARE_A1: Square = 0;
/// SQUARE_B1 represents square 'b1' of a chess board
pub const SQUARE_B1: Square = 1;
/// SQUARE_C1 represents square 'c1' of a chess board
pub const SQUARE_C1: Square = 2;
/// SQUARE_D1 represents square 'd1' of a chess board
pub const SQUARE_D1: Square = 3;
/// SQUARE_E1 represents square 'e1' of a chess board
pub const SQUARE_E1: Square = 4;
/// SQUARE_F1 represents square 'f1' of a chess board
pub const SQUARE_F1: Square = 5;
/// SQUARE_G1 represents square 'g1' of a chess board
pub const SQUARE_G1: Square = 6;
/// SQUARE_H1 represents square 'h1' of a chess board
pub const SQUARE_H1: Square = 7;
/// SQUARE_A2 represents square 'a2' of a chess board
pub const SQUARE_A2: Square = 8;
/// SQUARE_B2 represents square 'b2' of a chess board
pub const SQUARE_B2: Square = 9;
/// SQUARE_C2 represents square 'c2' of a chess board
pub const SQUARE_C2: Square = 10;
/// SQUARE_D2 represents square 'd2' of a chess board
pub const SQUARE_D2: Square = 11;
/// SQUARE_E2 represents square 'e2' of a chess board
pub const SQUARE_E2: Square = 12;
/// SQUARE_F2 represents square 'f2' of a chess board
pub const SQUARE_F2: Square = 13;
/// SQUARE_G2 represents square 'g2' of a chess board
pub const SQUARE_G2: Square = 14;
/// SQUARE_H2 represents square 'h2' of a chess board
pub const SQUARE_H2: Square = 15;
/// SQUARE_A3 represents square 'a3' of a chess board
pub const SQUARE_A3: Square = 16;
/// SQUARE_B3 represents square 'b3' of a chess board
pub const SQUARE_B3: Square = 17;
/// SQUARE_C3 represents square 'c3' of a chess board
pub const SQUARE_C3: Square = 18;
/// SQUARE_D3 represents square 'd3' of a chess board
pub const SQUARE_D3: Square = 19;
/// SQUARE_E3 represents square 'e3' of a chess board
pub const SQUARE_E3: Square = 20;
/// SQUARE_F3 represents square 'f3' of a chess board
pub const SQUARE_F3: Square = 21;
/// SQUARE_G3 represents square 'g3' of a chess board
pub const SQUARE_G3: Square = 22;
/// SQUARE_H3 represents square 'h3' of a chess board
pub const SQUARE_H3: Square = 23;
/// SQUARE_A4 represents square 'a4' of a chess board
pub const SQUARE_A4: Square = 24;
/// SQUARE_B4 represents square 'b4' of a chess board
pub const SQUARE_B4: Square = 25;
/// SQUARE_C4 represents square 'c4' of a chess board
pub const SQUARE_C4: Square = 26;
/// SQUARE_D4 represents square 'd4' of a chess board
pub const SQUARE_D4: Square = 27;
/// SQUARE_E4 represents square 'e4' of a chess board
pub const SQUARE_E4: Square = 28;
/// SQUARE_F4 represents square 'f4' of a chess board
pub const SQUARE_F4: Square = 29;
/// SQUARE_G4 represents square 'g4' of a chess board
pub const SQUARE_G4: Square = 30;
/// SQUARE_H4 represents square 'h4' of a chess board
pub const SQUARE_H4: Square = 31;
/// SQUARE_A5 represents square 'a5' of a chess board
pub const SQUARE_A5: Square = 32;
/// SQUARE_B5 represents square 'b5' of a chess board
pub const SQUARE_B5: Square = 33;
/// SQUARE_C5 represents square 'c5' of a chess board
pub const SQUARE_C5: Square = 34;
/// SQUARE_D5 represents square 'd5' of a chess board
pub const SQUARE_D5: Square = 35;
/// SQUARE_E5 represents square 'e5' of a chess board
pub const SQUARE_E5: Square = 36;
/// SQUARE_F5 represents square 'f5' of a chess board
pub const SQUARE_F5: Square = 37;
/// SQUARE_G5 represents square 'g5' of a chess board
pub const SQUARE_G5: Square = 38;
/// SQUARE_H5 represents square 'h5' of a chess board
pub const SQUARE_H5: Square = 39;
/// SQUARE_A6 represents square 'a6' of a chess board
pub const SQUARE_A6: Square = 40;
/// SQUARE_B6 represents square 'b6' of a chess board
pub const SQUARE_B6: Square = 41;
/// SQUARE_C6 represents square 'c6' of a chess board
pub const SQUARE_C6: Square = 42;
/// SQUARE_D6 represents square 'd6' of a chess board
pub const SQUARE_D6: Square = 43;
/// SQUARE_E6 represents square 'e6' of a chess board
pub const SQUARE_E6: Square = 44;
/// SQUARE_F6 represents square 'f6' of a chess board
pub const SQUARE_F6: Square = 45;
/// SQUARE_G6 represents square 'g6' of a chess board
pub const SQUARE_G6: Square = 46;
/// SQUARE_H6 represents square 'h6' of a chess board
pub const SQUARE_H6: Square = 47;
/// SQUARE_A7 represents square 'a7' of a chess board
pub const SQUARE_A7: Square = 48;
/// SQUARE_B7 represents square 'b7' of a chess board
pub const SQUARE_B7: Square = 49;
/// SQUARE_C7 represents square 'c7' of a chess board
pub const SQUARE_C7: Square = 50;
/// SQUARE_D7 represents square 'd7' of a chess board
pub const SQUARE_D7: Square = 51;
/// SQUARE_E7 represents square 'e7' of a chess board
pub const SQUARE_E7: Square = 52;
/// SQUARE_F7 represents square 'f7' of a chess board
pub const SQUARE_F7: Square = 53;
/// SQUARE_G7 represents square 'g7' of a chess board
pub const SQUARE_G7: Square = 54;
/// SQUARE_H7 represents square 'h7' of a chess board
pub const SQUARE_H7: Square = 55;
/// SQUARE_A8 represents square 'a8' of a chess board
pub const SQUARE_A8: Square = 56;
/// SQUARE_B8 represents square 'b8' of a chess board
pub const SQUARE_B8: Square = 57;
/// SQUARE_C8 represents square 'c8' of a chess board
pub const SQUARE_C8: Square = 58;
/// SQUARE_D8 represents square 'd8' of a chess board
pub const SQUARE_D8: Square = 59;
/// SQUARE_E8 represents square 'e8' of a chess board
pub const SQUARE_E8: Square = 60;
/// SQUARE_F8 represents square 'f8' of a chess board
pub const SQUARE_F8: Square = 61;
/// SQUARE_G8 represents square 'g8' of a chess board
pub const SQUARE_G8: Square = 62;
/// SQUARE_H8 represents square 'h8' of a chess board
pub const SQUARE_H8: Square = 63;

/// FILE_NAMES maps a file to a file name
pub const FILE_NAMES: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

/// constructs a Square from rank and file
pub fn rank_file(rank: Rank, file: File) -> Square {
    rank * NUM_FILES + file
}

/// Square trait adds methods to a Square
pub trait SquareTrait {
    /// returns the rank of the square
    fn rank(self) -> Rank;
    /// returns the file of the square
    fn file(self) -> File;
    /// returns the UCI representation of the square
    fn uci(self) -> String;
    /// returns an otherwise empty bitboard with the bit for this square set
    fn bitboard(self) -> Bitboard;
    /// adds a delta to a square and returns the resulting square
    /// together with a bool indicating whether adding was a success
    fn add_delta(self, delta: &Delta) -> (Square, bool);
    /// same as add_delta, but also fails if the resulting square is occupied
    fn add_delta_occup(self, delta: &Delta, occup: Bitboard) -> (Square, bool);
}

/// SquareTrait adds methods to a Square
impl SquareTrait for Square {
    /// returns the rank of the square
    fn rank(self) -> Rank {
        self >> RANK_SHIFT
    }

    /// returns the file of the square
    fn file(self) -> File {
        self & FILE_MASK
    }

    /// returns the UCI representation of the square
    fn uci(self) -> String {
        format!("{}{}", FILE_NAMES[self.file()], self.rank() + 1)
    }

    /// returns an otherwise empty bitboard with the bit for this square set
    fn bitboard(self) -> Bitboard {
        1 << (LAST_FILE - self.file()) + self.rank() * NUM_FILES
    }

    /// adds a delta to a square and returns the resulting square
    /// together with a bool indicating whether adding was a success
    fn add_delta(self, delta: &Delta) -> (Square, bool) {
        let file = self.file();
        let rank = self.rank();
        match delta {
            Delta::N => {
                return if rank > ONE_BEFORE_LAST_RANK {
                    (0, false)
                } else {
                    (rank_file(rank + 1, file), true)
                }
            }
            Delta::NE => {
                return if rank > ONE_BEFORE_LAST_RANK || file > ONE_BEFORE_LAST_FILE {
                    (0, false)
                } else {
                    (rank_file(rank + 1, file + 1), true)
                }
            }
            Delta::NNE => {
                return if rank > TWO_BEFORE_LAST_RANK || file > ONE_BEFORE_LAST_FILE {
                    (0, false)
                } else {
                    (rank_file(rank + 2, file + 1), true)
                }
            }
            Delta::NEE => {
                return if rank > ONE_BEFORE_LAST_RANK || file > TWO_BEFORE_LAST_FILE {
                    (0, false)
                } else {
                    (rank_file(rank + 1, file + 2), true)
                }
            }
            Delta::E => {
                return if file > ONE_BEFORE_LAST_FILE {
                    (0, false)
                } else {
                    (rank_file(rank, file + 1), true)
                }
            }
            Delta::SE => {
                return if rank < 1 || file > ONE_BEFORE_LAST_FILE {
                    (0, false)
                } else {
                    (rank_file(rank - 1, file + 1), true)
                }
            }
            Delta::SEE => {
                return if rank < 1 || file > TWO_BEFORE_LAST_FILE {
                    (0, false)
                } else {
                    (rank_file(rank - 1, file + 2), true)
                }
            }
            Delta::SSE => {
                return if rank < 2 || file > ONE_BEFORE_LAST_FILE {
                    (0, false)
                } else {
                    (rank_file(rank - 2, file + 1), true)
                }
            }
            Delta::S => {
                return if rank < 1 {
                    (0, false)
                } else {
                    (rank_file(rank - 1, file), true)
                }
            }
            Delta::SW => {
                return if rank < 1 || file < 1 {
                    (0, false)
                } else {
                    (rank_file(rank - 1, file - 1), true)
                }
            }
            Delta::SSW => {
                return if rank < 2 || file < 1 {
                    (0, false)
                } else {
                    (rank_file(rank - 2, file - 1), true)
                }
            }
            Delta::SWW => {
                return if rank < 1 || file < 2 {
                    (0, false)
                } else {
                    (rank_file(rank - 1, file - 2), true)
                }
            }
            Delta::W => {
                return if file < 1 {
                    (0, false)
                } else {
                    (rank_file(rank, file - 1), true)
                }
            }
            Delta::NW => {
                return if rank > ONE_BEFORE_LAST_RANK || file < 1 {
                    (0, false)
                } else {
                    (rank_file(rank + 1, file - 1), true)
                }
            }
            Delta::NWW => {
                return if rank > ONE_BEFORE_LAST_RANK || file < 2 {
                    (0, false)
                } else {
                    (rank_file(rank + 1, file - 2), true)
                }
            }
            Delta::NNW => {
                return if rank > TWO_BEFORE_LAST_RANK || file < 1 {
                    (0, false)
                } else {
                    (rank_file(rank + 2, file - 1), true)
                }
            }
        }
    }

    /// same as add_delta, but also fails if the resulting square is occupied
    fn add_delta_occup(self, delta: &Delta, occup: Bitboard) -> (Square, bool) {
        let (sq, ok) = self.add_delta(delta);
        if !ok {
            return (0, false);
        }
        if sq.bitboard() & occup != 0 {
            return (0, false);
        }
        return (sq, true);
    }
}

/// returns jump attack bitboard from 4 deltas
pub fn jump_attack<T: DeltaBuffer>(sq: Square, deltas: &T, occup: Bitboard) -> Bitboard {
    let mut bb: Bitboard = 0;
    for i in 0..deltas.len() {
        let (test_sq, ok) = sq.add_delta_occup(deltas.get(i), occup);
        if ok {
            bb |= test_sq.bitboard();
        }
    }
    bb
}

/// returns sliding attack bitboard from deltas
pub fn sliding_attack<T: DeltaBuffer>(sq: Square, deltas: &T, occup: Bitboard) -> Bitboard {
    let mut bb: Bitboard = 0;
    for i in 0..deltas.len() {
        let mut test_sq = sq;
        loop {
            let (new_test_sq, ok) = test_sq.add_delta(deltas.get(i));
            if ok {
                test_sq = new_test_sq;
                bb |= test_sq.bitboard();
                if (test_sq.bitboard()) & occup != 0 {
                    break;
                }
            } else {
                break;
            }
        }
    }
    bb
}

/// AttackTable type records an attack bitboard for every square of a chess board
pub type AttackTable = [Bitboard; BOARD_AREA];

/// EMPTY_ATTACK_TABLE defines an empty attack table, useful for initializing attack tables
pub const EMPTY_ATTACK_TABLE: AttackTable = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

/// KNIGHT_ATTACK is the attack table of knight
pub static KNIGHT_ATTACK: Lazy<AttackTable> = Lazy::new(|| {
    let mut at = EMPTY_ATTACK_TABLE;
    for sq in 0..BOARD_AREA {
        at[sq] = jump_attack(sq, &KNIGHT_DELTAS, 0);
    }
    at
});
/// BISHOP_ATTACK is the attack table of bishop
pub static BISHOP_ATTACK: Lazy<AttackTable> = Lazy::new(|| {
    let mut at = EMPTY_ATTACK_TABLE;
    for sq in 0..BOARD_AREA {
        at[sq] = sliding_attack(sq, &BISHOP_DELTAS, 0);
    }
    at
});
/// BISHOP_MAGIC_ATTACK is the magic attack table of bishop
pub static BISHOP_MAGIC_ATTACK: Lazy<AttackTable> = Lazy::new(|| {
    let mut at = EMPTY_ATTACK_TABLE;
    for sq in 0..BOARD_AREA {
        at[sq] = magic_attack(sq, sliding_attack(sq, &BISHOP_DELTAS, 0));
    }
    at
});
/// ROOK_ATTACK is the attack table of rook
pub static ROOK_ATTACK: Lazy<AttackTable> = Lazy::new(|| {
    let mut at = EMPTY_ATTACK_TABLE;
    for sq in 0..BOARD_AREA {
        at[sq] = sliding_attack(sq, &ROOK_DELTAS, 0);
    }
    at
});
/// ROOK_MAGIC_ATTACK is the magic attack table of rook
pub static ROOK_MAGIC_ATTACK: Lazy<AttackTable> = Lazy::new(|| {
    let mut at = EMPTY_ATTACK_TABLE;
    for sq in 0..BOARD_AREA {
        at[sq] = magic_attack(sq, sliding_attack(sq, &ROOK_DELTAS, 0));
    }
    at
});
/// QUEEN_ATTACK is the attack table of queen
pub static QUEEN_ATTACK: Lazy<AttackTable> = Lazy::new(|| {
    let mut at = EMPTY_ATTACK_TABLE;
    for sq in 0..BOARD_AREA {
        at[sq] = sliding_attack(sq, &QUEEN_DELTAS, 0);
    }
    at
});
/// KING_ATTACK is the attack table of king
pub static KING_ATTACK: Lazy<AttackTable> = Lazy::new(|| {
    let mut at = EMPTY_ATTACK_TABLE;
    for sq in 0..BOARD_AREA {
        at[sq] = jump_attack(sq, &QUEEN_DELTAS, 0);
    }
    at
});
/// KING_AREA is the attack table of king plus king square
pub static KING_AREA: Lazy<AttackTable> = Lazy::new(|| {
    let mut at = EMPTY_ATTACK_TABLE;
    for sq in 0..BOARD_AREA {
        at[sq] = jump_attack(sq, &QUEEN_DELTAS, 0) | sq.bitboard();
    }
    at
});

/// translates an occupancy mask to partial occupancy of a mobility
pub fn translate_mask_to_occupancy(mask: usize, mobility: Bitboard) -> Bitboard {
    let mut occup: Bitboard = 0;
    let mut mob = mobility;
    let mut m = mask;
    loop {
        let (bb, ok) = mob.pop_bitboard();
        if ok {
            if m & 1 != 0 {
                occup |= bb;
            }
            m >>= 1;
        } else {
            return occup;
        }
    }
}

/// returns a random magic
pub fn new_magic() -> u64 {
    rand::thread_rng().gen::<u64>()
}

/// returns index of mobility in mobility table for a magic and shift
pub fn mobility_index(mobility: Bitboard, magic: u64, shift: usize) -> usize {
    let (result, _) = mobility.overflowing_mul(magic);
    (result >> (64 - shift)) as usize
}

/// detects collision of a magic applied to a mobility, returns true if there is a collision, false otherwise
pub fn detect_collision(magic: u64, shift: usize, mobility: Bitboard) -> bool {
    let variations = mobility.variation_count();
    let mut mapped = vec![(false, 0); 1 << shift];

    let mut mask: usize = 0;

    loop {
        if mask < variations {
            let occup = translate_mask_to_occupancy(mask, mobility);
            let index = mobility_index(occup, magic, shift);
            let (used, bb) = mapped[index];
            if used {
                if bb != occup {
                    return true;
                }
            } else {
                mapped[index] = (true, occup);
            }
            mask += 1;
        } else {
            return false;
        }
    }
}

/// tries to find magic for shift in certain number of tries for a mobility, returns a tuple of the found magic and a bool indicating success
pub fn find_magic_for_shift(shift: usize, mobility: Bitboard, max_tries: usize) -> (u64, bool) {
    for _ in 0..max_tries {
        let magic = new_magic();
        if !detect_collision(magic, shift, mobility) {
            return (magic, true);
        }
    }
    return (0, false);
}

/// tries to find magic and shift in certain number of tries per shift for a mobility, starting from a maximum shift, going to minimum shift, returns a tuple of the found magic and a bool indicating success
pub fn find_magic_and_shift(
    mobility: Bitboard,
    max_shift: usize,
    min_shift: usize,
    max_tries: usize,
) -> (u64, usize, bool) {
    let mut last_good_shift: usize = 0;
    let mut last_good_magic: u64 = 0;
    let mut has_magic = false;
    for i in 0..(max_shift - min_shift + 1) {
        let shift = max_shift - i;
        let (magic, ok) = find_magic_for_shift(shift, mobility, max_tries);
        if ok {
            last_good_shift = shift;
            last_good_magic = magic;
            has_magic = true;
        }
    }
    if has_magic {
        return (last_good_magic, last_good_shift, true);
    }
    return (0, 0, false);
}

/// return magic attack for an attack
pub fn magic_attack(sq: Square, attack: Bitboard) -> Bitboard {
    let mut mask = BITBOARD_MIDDLE;

    if sq.rank() == 0 {
        mask |= BITBOARD_RANK_1_MIDDLE;
    }
    if sq.rank() == LAST_RANK {
        mask |= BITBOARD_RANK_8_MIDDLE;
    }
    if sq.file() == 0 {
        mask |= BITBOARD_FILE_A_MIDDLE;
    }
    if sq.file() == LAST_FILE {
        mask |= BITBOARD_FILE_H_MIDDLE;
    }

    attack & mask
}

/// find and log magic and shift
pub fn log_find_magic_and_shift(
    bw: &mut BufWriter<std::fs::File>,
    sq: Square,
    attack: Bitboard,
    name: &str,
) {
    let data = format!(
        "{}\nmobility for {} at square {} , count {}\n\n",
        attack.pretty_print_string(),
        name,
        sq.uci(),
        attack.count_ones()
    );

    (*bw)
        .write_all(data.as_bytes())
        .expect("Unable to write data.");

    println!("{}", data);

    let result = find_magic_and_shift(attack, 14, if name == "bishop" { 5 } else { 10 }, 5000);

    if !result.2 {
        panic!("shift too large")
    }

    let data = format!(
        "magic kind {} square {} magic {:016X} shift {}\n\n",
        name,
        sq.uci(),
        result.0,
        result.1
    );

    (*bw)
        .write_all(data.as_bytes())
        .expect("Unable to write data.");

    println!("{}", data);
}

/// find and log all magics
pub fn find_and_log_magics() {
    let file = std::fs::File::create("magics.txt").expect("Error: Unable to create magics file.");

    let mut bw = BufWriter::new(file);

    for sq in 0..BOARD_AREA {
        log_find_magic_and_shift(&mut bw, sq, magic_attack(sq, ROOK_ATTACK[sq]), "rook");

        log_find_magic_and_shift(&mut bw, sq, magic_attack(sq, BISHOP_ATTACK[sq]), "bishop");
    }
}

/// MagicInfo records the magic and shift for a square
pub struct MagicInfo {
    sq: Square,
    magic: u64,
    shift: usize,
}

/// BISHOP_MAGICS records bishop magics
pub const BISHOP_MAGICS: [MagicInfo; BOARD_AREA] = [
    MagicInfo {
        sq: SQUARE_A1,
        magic: 0x0EEEE053F756C577,
        shift: 7,
    },
    MagicInfo {
        sq: SQUARE_B1,
        magic: 0x20A1752D5294E8AA,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_C1,
        magic: 0xFFAE7DCBB67E2AD8,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_D1,
        magic: 0x95E20936ACF52029,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_E1,
        magic: 0x8659660ABA92446A,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_F1,
        magic: 0xCB2205A8F212E3CD,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_G1,
        magic: 0x63095C5766AEAC33,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_H1,
        magic: 0x6A0A2CDED7FD32B6,
        shift: 7,
    },
    MagicInfo {
        sq: SQUARE_A2,
        magic: 0xF7AA6799A42C1974,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_B2,
        magic: 0x0EEBFAAE2308CDE6,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_C2,
        magic: 0x62F5D4C232917A29,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_D2,
        magic: 0x6511DA45FB6E605B,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_E2,
        magic: 0x1C1020709501F819,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_F2,
        magic: 0x9713D67F067E5B55,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_G2,
        magic: 0x4A9B28CAC8388396,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_H2,
        magic: 0x0D80340307A89DE3,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_A3,
        magic: 0xDFE4E245340681A7,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_B3,
        magic: 0x086E282C294CE76A,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_C3,
        magic: 0x7940D0C20066331C,
        shift: 8,
    },
    MagicInfo {
        sq: SQUARE_D3,
        magic: 0xB268CBA3523295A2,
        shift: 8,
    },
    MagicInfo {
        sq: SQUARE_E3,
        magic: 0xFA6472E0AEE3A9CD,
        shift: 8,
    },
    MagicInfo {
        sq: SQUARE_F3,
        magic: 0x4AB90CCE21C05AC4,
        shift: 8,
    },
    MagicInfo {
        sq: SQUARE_G3,
        magic: 0x19BBE314DB87C5A2,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_H3,
        magic: 0x2D8420D15430341A,
        shift: 5,
    },
    MagicInfo {
        sq: SQUARE_A4,
        magic: 0x002064D946035D06,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_B4,
        magic: 0x8F7EFC3826FCBC9E,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_C4,
        magic: 0xA5EFF6F3B79E13AC,
        shift: 9,
    },
    MagicInfo {
        sq: SQUARE_D4,
        magic: 0x979736BB1A2BBF3F,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_E4,
        magic: 0x47EF601CFFC5AA96,
        shift: 11,
    },
    MagicInfo {
        sq: SQUARE_F4,
        magic: 0xCBB12ECFB3AB9886,
        shift: 9,
    },
    MagicInfo {
        sq: SQUARE_G4,
        magic: 0x9F3DE5985071281C,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_H4,
        magic: 0xF7F070016C4BB3B1,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_A5,
        magic: 0x5B257CF070A81662,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_B5,
        magic: 0xDDDDF92F252AC221,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_C5,
        magic: 0x03B272448E17640F,
        shift: 8,
    },
    MagicInfo {
        sq: SQUARE_D5,
        magic: 0x344929A251A4A2B4,
        shift: 11,
    },
    MagicInfo {
        sq: SQUARE_E5,
        magic: 0xC59870B586EE336E,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_F5,
        magic: 0x0D10F3CD6C346800,
        shift: 8,
    },
    MagicInfo {
        sq: SQUARE_G5,
        magic: 0x6C4A636CFAEE0FC3,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_H5,
        magic: 0x1B7C2059AA3C501E,
        shift: 5,
    },
    MagicInfo {
        sq: SQUARE_A6,
        magic: 0xF44FA78502E05C46,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_B6,
        magic: 0xA605295DB4D8A494,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_C6,
        magic: 0x57293F7EEA9072FC,
        shift: 8,
    },
    MagicInfo {
        sq: SQUARE_D6,
        magic: 0x7A4B502FE080693E,
        shift: 8,
    },
    MagicInfo {
        sq: SQUARE_E6,
        magic: 0x840FE3E0F7C37EFD,
        shift: 8,
    },
    MagicInfo {
        sq: SQUARE_F6,
        magic: 0xC6553AE93DE5DEEB,
        shift: 8,
    },
    MagicInfo {
        sq: SQUARE_G6,
        magic: 0x186E0A1720748471,
        shift: 5,
    },
    MagicInfo {
        sq: SQUARE_H6,
        magic: 0x344586BD3800F615,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_A7,
        magic: 0xDA81874F48DE2873,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_B7,
        magic: 0xE750DB951630E01C,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_C7,
        magic: 0x8D7AC01104BE4239,
        shift: 5,
    },
    MagicInfo {
        sq: SQUARE_D7,
        magic: 0xD069455082F020EF,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_E7,
        magic: 0x0B12D09BE0A80D3B,
        shift: 5,
    },
    MagicInfo {
        sq: SQUARE_F7,
        magic: 0xF66D45FADE4750BF,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_G7,
        magic: 0xDEE2FEF5608BB86B,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_H7,
        magic: 0x3BC4110C30BC372E,
        shift: 5,
    },
    MagicInfo {
        sq: SQUARE_A8,
        magic: 0x030D2624DC9C2FAB,
        shift: 7,
    },
    MagicInfo {
        sq: SQUARE_B8,
        magic: 0x3BA0C0EE6C2C0185,
        shift: 5,
    },
    MagicInfo {
        sq: SQUARE_C8,
        magic: 0x0B7687D6A30FCDB0,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_D8,
        magic: 0x1A17F24EB26AC5B9,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_E8,
        magic: 0x70266A5B055D77F6,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_F8,
        magic: 0x8498203F0163A952,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_G8,
        magic: 0xC4E6130B3DBE1E26,
        shift: 6,
    },
    MagicInfo {
        sq: SQUARE_H8,
        magic: 0x9CB76E818B87114C,
        shift: 7,
    },
];

/// ROOK_MAGICS records rook magics
pub const ROOK_MAGICS: [MagicInfo; BOARD_AREA] = [
    MagicInfo {
        sq: SQUARE_A1,
        magic: 0xEB5BFB1511B7C572,
        shift: 14,
    },
    MagicInfo {
        sq: SQUARE_B1,
        magic: 0xC8608D0688E7A11A,
        shift: 13,
    },
    MagicInfo {
        sq: SQUARE_C1,
        magic: 0x3CE72520084C5AD7,
        shift: 13,
    },
    MagicInfo {
        sq: SQUARE_D1,
        magic: 0xB1CD151D5279C4E2,
        shift: 13,
    },
    MagicInfo {
        sq: SQUARE_E1,
        magic: 0x145254517D0A38C5,
        shift: 13,
    },
    MagicInfo {
        sq: SQUARE_F1,
        magic: 0x890F78296C5C6B67,
        shift: 14,
    },
    MagicInfo {
        sq: SQUARE_G1,
        magic: 0x535154D37FDA3758,
        shift: 13,
    },
    MagicInfo {
        sq: SQUARE_H1,
        magic: 0x6F10FA4A53DDDD67,
        shift: 14,
    },
    MagicInfo {
        sq: SQUARE_A2,
        magic: 0xFCF4667B997AD49A,
        shift: 13,
    },
    MagicInfo {
        sq: SQUARE_B2,
        magic: 0xB381EE06B8D760C9,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_C2,
        magic: 0xF49744D7EA7A7F45,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_D2,
        magic: 0x2FDDB993EF98801C,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_E2,
        magic: 0x52893ECADF61693E,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_F2,
        magic: 0x0CACE9126E294884,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_G2,
        magic: 0x780E224E32B8259B,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_H2,
        magic: 0x4ED7F0F359C79D1D,
        shift: 13,
    },
    MagicInfo {
        sq: SQUARE_A3,
        magic: 0x58FF574A6B17102C,
        shift: 13,
    },
    MagicInfo {
        sq: SQUARE_B3,
        magic: 0xC18E42DF30B60108,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_C3,
        magic: 0x82615CC2DC1619E7,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_D3,
        magic: 0x0AA0CFFF3CAB034F,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_E3,
        magic: 0x3AA209B910A2A2C0,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_F3,
        magic: 0x7D6D42D864ED4744,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_G3,
        magic: 0x1F49AC2C1FDA6D5B,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_H3,
        magic: 0x3B6D1A3A4AF92D50,
        shift: 13,
    },
    MagicInfo {
        sq: SQUARE_A4,
        magic: 0xE7C3B1D0B8EDF3A8,
        shift: 13,
    },
    MagicInfo {
        sq: SQUARE_B4,
        magic: 0xE01E4C628A791328,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_C4,
        magic: 0xD9918490A8516264,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_D4,
        magic: 0x4AFFDBD9881440CE,
        shift: 13,
    },
    MagicInfo {
        sq: SQUARE_E4,
        magic: 0x29D9E1F13D9B48A5,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_F4,
        magic: 0xD2647A2309B70AF5,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_G4,
        magic: 0xF9978681E00B17E0,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_H4,
        magic: 0xF1F3DDFDAF83D405,
        shift: 13,
    },
    MagicInfo {
        sq: SQUARE_A5,
        magic: 0x954BA31977E062BA,
        shift: 13,
    },
    MagicInfo {
        sq: SQUARE_B5,
        magic: 0xD52B1274D43F1A9C,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_C5,
        magic: 0xBE43F8A40D902543,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_D5,
        magic: 0x8866A7F07B184CA5,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_E5,
        magic: 0xF219EF680D77619C,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_F5,
        magic: 0x9BB75C3B8476F746,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_G5,
        magic: 0xAD1EE18B5B780265,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_H5,
        magic: 0xB6B44224206E74E5,
        shift: 13,
    },
    MagicInfo {
        sq: SQUARE_A6,
        magic: 0x649CDC1F34AEA2F6,
        shift: 13,
    },
    MagicInfo {
        sq: SQUARE_B6,
        magic: 0xFF83A9859BA534C4,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_C6,
        magic: 0x37CA319A4D50C97E,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_D6,
        magic: 0x858715E0CC1F8F7A,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_E6,
        magic: 0xE729AA5F024DF2C0,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_F6,
        magic: 0x1274960D5333E983,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_G6,
        magic: 0x4E78A790882C2806,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_H6,
        magic: 0x7C27B241F8825A5B,
        shift: 13,
    },
    MagicInfo {
        sq: SQUARE_A7,
        magic: 0x0DFC0F9386834FD8,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_B7,
        magic: 0x87342325FE073668,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_C7,
        magic: 0x850AE96248D88210,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_D7,
        magic: 0x8A8D8EA6E640F8F7,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_E7,
        magic: 0xC6AC009AB7852C97,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_F7,
        magic: 0x6D9B84E49D05E5D8,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_G7,
        magic: 0xF598D5B0F5881D70,
        shift: 11,
    },
    MagicInfo {
        sq: SQUARE_H7,
        magic: 0x1238BF08A6F9C38D,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_A8,
        magic: 0xCFC8C410148D3AF6,
        shift: 13,
    },
    MagicInfo {
        sq: SQUARE_B8,
        magic: 0x234F1593282FADBC,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_C8,
        magic: 0x6BCC4CA147847096,
        shift: 13,
    },
    MagicInfo {
        sq: SQUARE_D8,
        magic: 0xA7FF3C5F35FFF73A,
        shift: 13,
    },
    MagicInfo {
        sq: SQUARE_E8,
        magic: 0x6A4F4F1EC85934C6,
        shift: 13,
    },
    MagicInfo {
        sq: SQUARE_F8,
        magic: 0x7E23DAD717AC6081,
        shift: 13,
    },
    MagicInfo {
        sq: SQUARE_G8,
        magic: 0x7211D918B0800852,
        shift: 12,
    },
    MagicInfo {
        sq: SQUARE_H8,
        magic: 0x8588AC87AA8CA46A,
        shift: 13,
    },
];

/// returns the total number of magic look up table items
pub fn total_magic_space(magics: [MagicInfo; BOARD_AREA]) -> usize {
    let mut total = 0;
    for i in 0..BOARD_AREA {
        let mi = &magics[i];
        total += 1 << mi.shift;
    }
    total
}

/// BISHOP_MAGIC_UNITS tells the total number of bishop lookup table items
pub const BISHOP_MAGIC_UNITS: usize = 18976;
/// ROOK_MAGIC_UNITS tells the total number of rook lookup table items
pub const ROOK_MAGIC_UNITS: usize = 387072;

/// create magic lookup table
pub fn create_magic_lookup_table<T: DeltaBuffer>(
    mis: &[MagicInfo; BOARD_AREA],
    at: &AttackTable,
    deltas: &T,
) -> Vec<Vec<Bitboard>> {
    let mut table = vec![vec![0; 0]; BOARD_AREA];
    for sq in 0..BOARD_AREA {
        let mobility = at[sq];
        let variations = mobility.variation_count();

        let mut mask: usize = 0;

        let sq = mis[sq].sq; // just to use redundant sq for something

        let magic = mis[sq].magic;
        let shift = mis[sq].shift;

        table[sq] = vec![0; 1 << shift];

        loop {
            if mask < variations {
                let occup = translate_mask_to_occupancy(mask, mobility);
                let index = mobility_index(occup, magic, shift);

                table[sq][index] = sliding_attack(sq, deltas, occup);

                mask += 1;
            } else {
                break;
            }
        }
    }
    table
}

/// MAGIC_LOOKUP_BISHOP is the magic lookup table for bishop
pub static MAGIC_LOOKUP_BISHOP: Lazy<Vec<Vec<Bitboard>>> =
    Lazy::new(|| create_magic_lookup_table(&BISHOP_MAGICS, &BISHOP_MAGIC_ATTACK, &BISHOP_DELTAS));

/// MAGIC_LOOKUP_ROOK is the magic lookup table for rook
pub static MAGIC_LOOKUP_ROOK: Lazy<Vec<Vec<Bitboard>>> =
    Lazy::new(|| create_magic_lookup_table(&ROOK_MAGICS, &ROOK_MAGIC_ATTACK, &ROOK_DELTAS));

/// MoveGenNode lists the move generation modes
#[derive(Copy, Clone)]
pub enum MoveGenMode {
    Violent,
    Quiet,
    All,
}

/// returns sliding mobility
pub fn get_sliding_mobility(
    gen_mode: MoveGenMode,
    sq: Square,
    occup_us: Bitboard,
    occup_them: Bitboard,
    mis: &[MagicInfo; BOARD_AREA],
    at: &AttackTable,
    lookup_table: &Vec<Vec<Bitboard>>,
) -> Bitboard {
    let magic = mis[sq].magic;
    let shift = mis[sq].shift;
    let magic_occup = (occup_us | occup_them) & at[sq];
    let index = mobility_index(magic_occup, magic, shift);
    match gen_mode {
        MoveGenMode::All => lookup_table[sq][index] & !occup_us,
        MoveGenMode::Violent => lookup_table[sq][index] & occup_them,
        MoveGenMode::Quiet => lookup_table[sq][index] & !(occup_us | occup_them),
    }
}

/// returns bishop mobility
pub fn bishop_mobility(
    gen_mode: MoveGenMode,
    sq: Square,
    occup_us: Bitboard,
    occup_them: Bitboard,
) -> Bitboard {
    get_sliding_mobility(
        gen_mode,
        sq,
        occup_us,
        occup_them,
        &BISHOP_MAGICS,
        &BISHOP_MAGIC_ATTACK,
        &MAGIC_LOOKUP_BISHOP,
    )
}

/// returns rook mobility
pub fn rook_mobility(
    gen_mode: MoveGenMode,
    sq: Square,
    occup_us: Bitboard,
    occup_them: Bitboard,
) -> Bitboard {
    get_sliding_mobility(
        gen_mode,
        sq,
        occup_us,
        occup_them,
        &ROOK_MAGICS,
        &ROOK_MAGIC_ATTACK,
        &MAGIC_LOOKUP_ROOK,
    )
}

/// returns queen mobility
pub fn queen_mobility(
    gen_mode: MoveGenMode,
    sq: Square,
    occup_us: Bitboard,
    occup_them: Bitboard,
) -> Bitboard {
    bishop_mobility(gen_mode, sq, occup_us, occup_them)
        | rook_mobility(gen_mode, sq, occup_us, occup_them)
}
