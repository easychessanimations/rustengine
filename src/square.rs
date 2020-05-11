/// Rank type represents the rank of a square as an unsigned int
pub type Rank = usize;
/// Rank type represents the file of a square as an unsigned int
pub type File = usize;
/// Square type represents a square of a chess board as an unsigned int
pub type Square = usize;

/// NUM_RANKS tells the number of ranks of a chess board
pub const NUM_RANKS: usize = 8;
/// NUM_RANKS tells the number of files of a chess board
pub const NUM_FILES: usize = 8;

/// RANK_SHIFT tells the position of the rank in bits within a square
pub const RANK_SHIFT: usize = 3;
/// FILE_MASK is a but mask that has bits set that represent the file within a square
pub const FILE_MASK: usize = (1 << RANK_SHIFT) - 1;

pub const RANK_1: Rank = 0;
pub const RANK_2: Rank = 1;
pub const RANK_3: Rank = 2;
pub const RANK_4: Rank = 3;
pub const RANK_5: Rank = 4;
pub const RANK_6: Rank = 5;
pub const RANK_7: Rank = 6;
pub const RANK_8: Rank = 7;

pub const FILE_A: File = 0;
pub const FILE_B: File = 1;
pub const FILE_C: File = 2;
pub const FILE_D: File = 3;
pub const FILE_E: File = 4;
pub const FILE_F: File = 5;
pub const FILE_G: File = 6;
pub const FILE_H: File = 7;

pub const SQUARE_A1: Square = 0;
pub const SQUARE_B1: Square = 1;
pub const SQUARE_C1: Square = 2;
pub const SQUARE_D1: Square = 3;
pub const SQUARE_E1: Square = 4;
pub const SQUARE_F1: Square = 5;
pub const SQUARE_G1: Square = 6;
pub const SQUARE_H1: Square = 7;
pub const SQUARE_A2: Square = 8;
pub const SQUARE_B2: Square = 9;
pub const SQUARE_C2: Square = 10;
pub const SQUARE_D2: Square = 11;
pub const SQUARE_E2: Square = 12;
pub const SQUARE_F2: Square = 13;
pub const SQUARE_G2: Square = 14;
pub const SQUARE_H2: Square = 15;
pub const SQUARE_A3: Square = 16;
pub const SQUARE_B3: Square = 17;
pub const SQUARE_C3: Square = 18;
pub const SQUARE_D3: Square = 19;
pub const SQUARE_E3: Square = 20;
pub const SQUARE_F3: Square = 21;
pub const SQUARE_G3: Square = 22;
pub const SQUARE_H3: Square = 23;
pub const SQUARE_A4: Square = 24;
pub const SQUARE_B4: Square = 25;
pub const SQUARE_C4: Square = 26;
pub const SQUARE_D4: Square = 27;
pub const SQUARE_E4: Square = 28;
pub const SQUARE_F4: Square = 29;
pub const SQUARE_G4: Square = 30;
pub const SQUARE_H4: Square = 31;
pub const SQUARE_A5: Square = 32;
pub const SQUARE_B5: Square = 33;
pub const SQUARE_C5: Square = 34;
pub const SQUARE_D5: Square = 35;
pub const SQUARE_E5: Square = 36;
pub const SQUARE_F5: Square = 37;
pub const SQUARE_G5: Square = 38;
pub const SQUARE_H5: Square = 39;
pub const SQUARE_A6: Square = 40;
pub const SQUARE_B6: Square = 41;
pub const SQUARE_C6: Square = 42;
pub const SQUARE_D6: Square = 43;
pub const SQUARE_E6: Square = 44;
pub const SQUARE_F6: Square = 45;
pub const SQUARE_G6: Square = 46;
pub const SQUARE_H6: Square = 47;
pub const SQUARE_A7: Square = 48;
pub const SQUARE_B7: Square = 49;
pub const SQUARE_C7: Square = 50;
pub const SQUARE_D7: Square = 51;
pub const SQUARE_E7: Square = 52;
pub const SQUARE_F7: Square = 53;
pub const SQUARE_G7: Square = 54;
pub const SQUARE_H7: Square = 55;
pub const SQUARE_A8: Square = 56;
pub const SQUARE_B8: Square = 57;
pub const SQUARE_C8: Square = 58;
pub const SQUARE_D8: Square = 59;
pub const SQUARE_E8: Square = 60;
pub const SQUARE_F8: Square = 61;
pub const SQUARE_G8: Square = 62;
pub const SQUARE_H8: Square = 63;

/// FILE_NAMES maps a file to a file name
const FILE_NAMES: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

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
}
