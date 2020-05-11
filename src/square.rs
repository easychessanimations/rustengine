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

/// SQUARE_A1 represents rank square 'a1' of a chess board
pub const SQUARE_A1: Square = 0;
/// SQUARE_B1 represents rank square 'b1' of a chess board
pub const SQUARE_B1: Square = 1;
/// SQUARE_C1 represents rank square 'c1' of a chess board
pub const SQUARE_C1: Square = 2;
/// SQUARE_D1 represents rank square 'd1' of a chess board
pub const SQUARE_D1: Square = 3;
/// SQUARE_E1 represents rank square 'e1' of a chess board
pub const SQUARE_E1: Square = 4;
/// SQUARE_F1 represents rank square 'f1' of a chess board
pub const SQUARE_F1: Square = 5;
/// SQUARE_G1 represents rank square 'g1' of a chess board
pub const SQUARE_G1: Square = 6;
/// SQUARE_H1 represents rank square 'h1' of a chess board
pub const SQUARE_H1: Square = 7;
/// SQUARE_A2 represents rank square 'a2' of a chess board
pub const SQUARE_A2: Square = 8;
/// SQUARE_B2 represents rank square 'b2' of a chess board
pub const SQUARE_B2: Square = 9;
/// SQUARE_C2 represents rank square 'c2' of a chess board
pub const SQUARE_C2: Square = 10;
/// SQUARE_D2 represents rank square 'd2' of a chess board
pub const SQUARE_D2: Square = 11;
/// SQUARE_E2 represents rank square 'e2' of a chess board
pub const SQUARE_E2: Square = 12;
/// SQUARE_F2 represents rank square 'f2' of a chess board
pub const SQUARE_F2: Square = 13;
/// SQUARE_G2 represents rank square 'g2' of a chess board
pub const SQUARE_G2: Square = 14;
/// SQUARE_H2 represents rank square 'h2' of a chess board
pub const SQUARE_H2: Square = 15;
/// SQUARE_A3 represents rank square 'a3' of a chess board
pub const SQUARE_A3: Square = 16;
/// SQUARE_B3 represents rank square 'b3' of a chess board
pub const SQUARE_B3: Square = 17;
/// SQUARE_C3 represents rank square 'c3' of a chess board
pub const SQUARE_C3: Square = 18;
/// SQUARE_D3 represents rank square 'd3' of a chess board
pub const SQUARE_D3: Square = 19;
/// SQUARE_E3 represents rank square 'e3' of a chess board
pub const SQUARE_E3: Square = 20;
/// SQUARE_F3 represents rank square 'f3' of a chess board
pub const SQUARE_F3: Square = 21;
/// SQUARE_G3 represents rank square 'g3' of a chess board
pub const SQUARE_G3: Square = 22;
/// SQUARE_H3 represents rank square 'h3' of a chess board
pub const SQUARE_H3: Square = 23;
/// SQUARE_A4 represents rank square 'a4' of a chess board
pub const SQUARE_A4: Square = 24;
/// SQUARE_B4 represents rank square 'b4' of a chess board
pub const SQUARE_B4: Square = 25;
/// SQUARE_C4 represents rank square 'c4' of a chess board
pub const SQUARE_C4: Square = 26;
/// SQUARE_D4 represents rank square 'd4' of a chess board
pub const SQUARE_D4: Square = 27;
/// SQUARE_E4 represents rank square 'e4' of a chess board
pub const SQUARE_E4: Square = 28;
/// SQUARE_F4 represents rank square 'f4' of a chess board
pub const SQUARE_F4: Square = 29;
/// SQUARE_G4 represents rank square 'g4' of a chess board
pub const SQUARE_G4: Square = 30;
/// SQUARE_H4 represents rank square 'h4' of a chess board
pub const SQUARE_H4: Square = 31;
/// SQUARE_A5 represents rank square 'a5' of a chess board
pub const SQUARE_A5: Square = 32;
/// SQUARE_B5 represents rank square 'b5' of a chess board
pub const SQUARE_B5: Square = 33;
/// SQUARE_C5 represents rank square 'c5' of a chess board
pub const SQUARE_C5: Square = 34;
/// SQUARE_D5 represents rank square 'd5' of a chess board
pub const SQUARE_D5: Square = 35;
/// SQUARE_E5 represents rank square 'e5' of a chess board
pub const SQUARE_E5: Square = 36;
/// SQUARE_F5 represents rank square 'f5' of a chess board
pub const SQUARE_F5: Square = 37;
/// SQUARE_G5 represents rank square 'g5' of a chess board
pub const SQUARE_G5: Square = 38;
/// SQUARE_H5 represents rank square 'h5' of a chess board
pub const SQUARE_H5: Square = 39;
/// SQUARE_A6 represents rank square 'a6' of a chess board
pub const SQUARE_A6: Square = 40;
/// SQUARE_B6 represents rank square 'b6' of a chess board
pub const SQUARE_B6: Square = 41;
/// SQUARE_C6 represents rank square 'c6' of a chess board
pub const SQUARE_C6: Square = 42;
/// SQUARE_D6 represents rank square 'd6' of a chess board
pub const SQUARE_D6: Square = 43;
/// SQUARE_E6 represents rank square 'e6' of a chess board
pub const SQUARE_E6: Square = 44;
/// SQUARE_F6 represents rank square 'f6' of a chess board
pub const SQUARE_F6: Square = 45;
/// SQUARE_G6 represents rank square 'g6' of a chess board
pub const SQUARE_G6: Square = 46;
/// SQUARE_H6 represents rank square 'h6' of a chess board
pub const SQUARE_H6: Square = 47;
/// SQUARE_A7 represents rank square 'a7' of a chess board
pub const SQUARE_A7: Square = 48;
/// SQUARE_B7 represents rank square 'b7' of a chess board
pub const SQUARE_B7: Square = 49;
/// SQUARE_C7 represents rank square 'c7' of a chess board
pub const SQUARE_C7: Square = 50;
/// SQUARE_D7 represents rank square 'd7' of a chess board
pub const SQUARE_D7: Square = 51;
/// SQUARE_E7 represents rank square 'e7' of a chess board
pub const SQUARE_E7: Square = 52;
/// SQUARE_F7 represents rank square 'f7' of a chess board
pub const SQUARE_F7: Square = 53;
/// SQUARE_G7 represents rank square 'g7' of a chess board
pub const SQUARE_G7: Square = 54;
/// SQUARE_H7 represents rank square 'h7' of a chess board
pub const SQUARE_H7: Square = 55;
/// SQUARE_A8 represents rank square 'a8' of a chess board
pub const SQUARE_A8: Square = 56;
/// SQUARE_B8 represents rank square 'b8' of a chess board
pub const SQUARE_B8: Square = 57;
/// SQUARE_C8 represents rank square 'c8' of a chess board
pub const SQUARE_C8: Square = 58;
/// SQUARE_D8 represents rank square 'd8' of a chess board
pub const SQUARE_D8: Square = 59;
/// SQUARE_E8 represents rank square 'e8' of a chess board
pub const SQUARE_E8: Square = 60;
/// SQUARE_F8 represents rank square 'f8' of a chess board
pub const SQUARE_F8: Square = 61;
/// SQUARE_G8 represents rank square 'g8' of a chess board
pub const SQUARE_G8: Square = 62;
/// SQUARE_H8 represents rank square 'h8' of a chess board
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
