use crate::bitboard::*;
use crate::constants::*;

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

/// constructs a Square from rank and file
pub fn rank_file(rank: Rank, file: File) -> Square {
    rank * NUM_FILES + file
}

/// Square trait adds methods to a Square
pub trait SquareTrait {
    /// creates Square from uci
    fn from_uci(uci: String) -> (Square, bool);
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
    /// creates Square from uci    
    fn from_uci(uci: String) -> (Square, bool) {
        if uci.len() != 2 {
            return (SQUARE_A1, false);
        }
        let file = match &uci[0..1] {
            "a" => 0,
            "b" => 1,
            "c" => 2,
            "d" => 3,
            "e" => 4,
            "f" => 5,
            "g" => 6,
            "h" => 7,
            _ => panic!("invalid file {:?}", &uci[0..1]),
        };
        let rank = match &uci[1..2] {
            "1" => 0,
            "2" => 1,
            "3" => 2,
            "4" => 3,
            "5" => 4,
            "6" => 5,
            "7" => 6,
            "8" => 7,
            _ => panic!("invalid rank {:?}", &uci[1..2]),
        };
        (rank * NUM_FILES + file, true)
    }
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

/// returns jump attack bitboard from deltas
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
    pub sq: Square,
    pub magic: u64,
    pub shift: usize,
}

/// returns the total number of magic look up table items
pub fn total_magic_space(magics: [MagicInfo; BOARD_AREA]) -> usize {
    let mut total = 0;
    for i in 0..BOARD_AREA {
        let mi = &magics[i];
        total += 1 << mi.shift;
    }
    total
}

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
