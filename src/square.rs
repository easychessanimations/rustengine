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
            let (new_test_sq, ok) = test_sq.add_delta_occup(deltas.get(i), occup);
            if ok {
                test_sq = new_test_sq;
                bb |= test_sq.bitboard();
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
/// ROOK_ATTACK is the attack table of rook
pub static ROOK_ATTACK: Lazy<AttackTable> = Lazy::new(|| {
    let mut at = EMPTY_ATTACK_TABLE;
    for sq in 0..BOARD_AREA {
        at[sq] = sliding_attack(sq, &ROOK_DELTAS, 0);
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

    let result = find_magic_and_shift(attack, 20, 7, 100);

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

pub fn find_and_log_magics() {
    let file = std::fs::File::create("magics.txt").expect("Error: Unable to create magics file.");

    let mut bw = BufWriter::new(file);

    for sq in 0..BOARD_AREA {
        log_find_magic_and_shift(&mut bw, sq, magic_attack(sq, ROOK_ATTACK[sq]), "rook");

        log_find_magic_and_shift(&mut bw, sq, magic_attack(sq, BISHOP_ATTACK[sq]), "bishop");
    }
}
