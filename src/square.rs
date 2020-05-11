pub const RANK_SHIFT: usize = 3;
pub const FILE_MASK: usize = (1 << RANK_SHIFT) - 1;

pub const SQUARE_A1: usize = 0;
pub const SQUARE_B1: usize = 1;
pub const SQUARE_C1: usize = 2;
pub const SQUARE_D1: usize = 3;
pub const SQUARE_E1: usize = 4;
pub const SQUARE_F1: usize = 5;
pub const SQUARE_G1: usize = 6;
pub const SQUARE_H1: usize = 7;
pub const SQUARE_A2: usize = 8;
pub const SQUARE_B2: usize = 9;
pub const SQUARE_C2: usize = 10;
pub const SQUARE_D2: usize = 11;
pub const SQUARE_E2: usize = 12;
pub const SQUARE_F2: usize = 13;
pub const SQUARE_G2: usize = 14;
pub const SQUARE_H2: usize = 15;
pub const SQUARE_A3: usize = 16;
pub const SQUARE_B3: usize = 17;
pub const SQUARE_C3: usize = 18;
pub const SQUARE_D3: usize = 19;
pub const SQUARE_E3: usize = 20;
pub const SQUARE_F3: usize = 21;
pub const SQUARE_G3: usize = 22;
pub const SQUARE_H3: usize = 23;
pub const SQUARE_A4: usize = 24;
pub const SQUARE_B4: usize = 25;
pub const SQUARE_C4: usize = 26;
pub const SQUARE_D4: usize = 27;
pub const SQUARE_E4: usize = 28;
pub const SQUARE_F4: usize = 29;
pub const SQUARE_G4: usize = 30;
pub const SQUARE_H4: usize = 31;
pub const SQUARE_A5: usize = 32;
pub const SQUARE_B5: usize = 33;
pub const SQUARE_C5: usize = 34;
pub const SQUARE_D5: usize = 35;
pub const SQUARE_E5: usize = 36;
pub const SQUARE_F5: usize = 37;
pub const SQUARE_G5: usize = 38;
pub const SQUARE_H5: usize = 39;
pub const SQUARE_A6: usize = 40;
pub const SQUARE_B6: usize = 41;
pub const SQUARE_C6: usize = 42;
pub const SQUARE_D6: usize = 43;
pub const SQUARE_E6: usize = 44;
pub const SQUARE_F6: usize = 45;
pub const SQUARE_G6: usize = 46;
pub const SQUARE_H6: usize = 47;
pub const SQUARE_A7: usize = 48;
pub const SQUARE_B7: usize = 49;
pub const SQUARE_C7: usize = 50;
pub const SQUARE_D7: usize = 51;
pub const SQUARE_E7: usize = 52;
pub const SQUARE_F7: usize = 53;
pub const SQUARE_G7: usize = 54;
pub const SQUARE_H7: usize = 55;
pub const SQUARE_A8: usize = 56;
pub const SQUARE_B8: usize = 57;
pub const SQUARE_C8: usize = 58;
pub const SQUARE_D8: usize = 59;
pub const SQUARE_E8: usize = 60;
pub const SQUARE_F8: usize = 61;
pub const SQUARE_G8: usize = 62;
pub const SQUARE_H8: usize = 63;

const FILE_NAMES: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

/// Square trait adds functions to an u8 that allow treating it as a chess square
pub trait Square {
    /// returns the rank of the square
    fn rank(self) -> usize;
    /// returns the file of the square
    fn file(self) -> usize;
    /// returns the UCI representation of the square
    fn uci(self) -> String;
}

impl Square for usize {
    /// returns the rank of the square
    fn rank(self) -> usize {
        self >> RANK_SHIFT
    }

    /// returns the file of the square
    fn file(self) -> usize {
        self & FILE_MASK
    }

    /// returns the UCI representation of the square
    fn uci(self) -> String {
        format!("{}{}", FILE_NAMES[self.file()], self.rank() + 1)
    }
}