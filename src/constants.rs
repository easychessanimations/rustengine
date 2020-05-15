use crate::bitboard::*;
use crate::piece::*;
use crate::square::*;
use crate::state::*;

/// BISHOP_MAGIC_UNITS tells the total number of bishop lookup table items
pub const BISHOP_MAGIC_UNITS: usize = 18976;
/// ROOK_MAGIC_UNITS tells the total number of rook lookup table items
pub const ROOK_MAGIC_UNITS: usize = 387072;

/// NUM_LANCERS tells the number of possible lancer direction variations
pub const NUM_LANCERS: usize = 8;

/// PAWN_START_RANKS tells the pawn start rank for color
pub const PAWN_START_RANKS: [Rank; 2] = [RANK_7, RANK_2];

/// LANCER_DELTAS lists lancer deltas by direction
pub const LANCER_DELTAS: [Delta; NUM_LANCERS] = [
    Delta::N,
    Delta::NE,
    Delta::E,
    Delta::SE,
    Delta::S,
    Delta::SW,
    Delta::W,
    Delta::NW,
];

/// SQUARE_SIZE_IN_BITS tells the number of bits used to represent a square
pub const SQUARE_SIZE_IN_BITS: usize = 6;

/// SQUARE_MASK can be used to mask the bits representing a square
pub const SQUARE_MASK: u32 = (1 << SQUARE_SIZE_IN_BITS) - 1;

/// FROM_SQ_SHIFT is the shift of from square in Move
pub const FROM_SQ_SHIFT: usize = 0;
/// TO_SQ_SHIFT is the shift of to square in Move
pub const TO_SQ_SHIFT: usize = SQUARE_SIZE_IN_BITS;

/// EMPTY_CASTLING_RIGHT represents an empty castling right
pub const EMPTY_CASTLING_RIGHT: CastlingRight = CastlingRight { can_castle: false };

/// EMPTY_COLOR_CASTLING_RIGHTS represents empty castling rights for a color
pub const EMPTY_COLOR_CASTLING_RIGHTS: ColorCastlingRights = ColorCastlingRights {
    rights: [EMPTY_CASTLING_RIGHT, EMPTY_CASTLING_RIGHT],
};

/// KING_SIDE is the index for king side castling right
pub const KING_SIDE: usize = 0;
/// QUEEN_SIDE is the index for queen side castling right
pub const QUEEN_SIDE: usize = 1;

/// NUM_VARIANTS tells the number of possible variants
pub const NUM_VARIANTS: usize = 3;

/// VARIANT_STANDARD is the index for Standard variant
pub const VARIANT_STANDARD: Variant = 0;
/// VARIANT_EIGHTPIECE is the index for Eightpiece variant
pub const VARIANT_EIGHTPIECE: Variant = 1;
/// VARIANT_ATOMIC is the index for Atomic variant
pub const VARIANT_ATOMIC: Variant = 2;

/// DEFAULT_VARIANT tells the default variant
pub const DEFAULT_VARIANT: Variant = VARIANT_EIGHTPIECE;

/// VARIANT_INFOS records information for all variants
pub const VARIANT_INFOS: [VariantInfo; NUM_VARIANTS] = [
    VariantInfo {
        // standard
        start_fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        display_name: "Standard",
    },
    VariantInfo {
        // eightpiece
        start_fen: "jlsesqkbnr/pppppppp/8/8/8/8/PPPPPPPP/JLneSQKBNR w KQkq - 0 1 -",
        display_name: "Eightpiece",
    },
    VariantInfo {
        // atomic
        start_fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        display_name: "Atomic",
    },
];

/// MAX_STATES tells the maximum number of states in a LinearGaeme
pub const MAX_STATES: usize = 100;

/// EMPTRY_REP represents and empty chess board
pub const EMPTY_REP: [Piece; BOARD_AREA] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

/// FIGURE_ARRAY_SIZE tells the number of possible figures
pub const FIGURE_ARRAY_SIZE: usize = 18;

/// EMTPY_FIGURE_BITBOARDS represents an empty bitboard for all possible figures
pub const EMTPY_FIGURE_BITBOARDS: [Bitboard; FIGURE_ARRAY_SIZE] =
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

/// BITBOARD_MIDDLE represents the bitboard for the middle of the board
pub const BITBOARD_MIDDLE: Bitboard = 0x007E7E7E7E7E7E00;
/// BITBOARD_RANK_8 represents the bitboard for the 8th rank of the board
pub const BITBOARD_RANK_8: Bitboard = 0xff00000000000000;
/// BITBOARD_RANK_8_MIDDLE represents the bitboard for the middle of the 8th rank of the board
pub const BITBOARD_RANK_8_MIDDLE: Bitboard = 0x7E00000000000000;
/// BITBOARD_RANK_1 represents the bitboard for the 1st rank of the board
pub const BITBOARD_RANK_1: Bitboard = 0x00000000000000ff;
/// BITBOARD_RANK_1_MIDDLE represents the bitboard for the middle of the 1st rank of the board
pub const BITBOARD_RANK_1_MIDDLE: Bitboard = 0x000000000000007E;
/// BITBOARD_FILE_A represents file 'a' of the board
pub const BITBOARD_FILE_A: Bitboard = 0x8080808080808080;
/// BITBOARD_FILE_A_MIDDLE represents the middle of file 'a' of the board
pub const BITBOARD_FILE_A_MIDDLE: Bitboard = 0x0080808080808000;
/// BITBOARD_FILE_H represents file 'h' of the board
pub const BITBOARD_FILE_H: Bitboard = 0x0101010101010101;
/// BITBOARD_FILE_H_MIDDLE represents the middle of file 'h' of the board
pub const BITBOARD_FILE_H_MIDDLE: Bitboard = 0x0001010101010100;

/// BLACK represents black chess color
pub const BLACK: Color = 0;
/// WHITE represents white chess color
pub const WHITE: Color = 1;

/// PIECE_FEN_SYMBOLS maps a piece to its fen symbol
pub const PIECE_FEN_SYMBOLS: [&str; 36] = [
    ".", ".", "p", "P", "n", "N", "b", "B", "r", "R", "q", "Q", "k", "K", "l", "L", "ln", "Ln",
    "lne", "Lne", "le", "Le", "lse", "Lse", "ls", "Ls", "lsw", "Lsw", "lw", "Lw", "lnw", "Lnw",
    "s", "S", "j", "J",
];

/// NO_FIGURE represents no piece on a given square
pub const NO_PIECE: Piece = 0;

/// NO_FIGURE represents no figure on a given square
pub const NO_FIGURE: Figure = 0;
/// PAWN represents chess figure 'pawn'
pub const PAWN: Figure = 1;
/// KNIGHT represents chess figure 'knight'
pub const KNIGHT: Figure = 2;
/// BISHOP represents chess figure 'bishop'
pub const BISHOP: Figure = 3;
/// ROOK represents chess figure 'rook'
pub const ROOK: Figure = 4;
/// QUEEN represents chess figure 'queen'
pub const QUEEN: Figure = 5;
/// KING represents chess figure 'king'
pub const KING: Figure = 6;
/// LANCER represents chess figure 'lancer'
pub const LANCER: Figure = 7;
/// LANCERN represents chess figure 'lancern'
pub const LANCERN: Figure = 8;
/// LANCERNE represents chess figure 'lancerne'
pub const LANCERNE: Figure = 9;
/// LANCERE represents chess figure 'lancere'
pub const LANCERE: Figure = 10;
/// LANCERSE represents chess figure 'lancerse'
pub const LANCERSE: Figure = 11;
/// LANCERS represents chess figure 'lancers'
pub const LANCERS: Figure = 12;
/// LANCERSW represents chess figure 'lancersw'
pub const LANCERSW: Figure = 13;
/// LANCERW represents chess figure 'lancerw'
pub const LANCERW: Figure = 14;
/// LANCERNW represents chess figure 'lancernw'
pub const LANCERNW: Figure = 15;
/// SENTRY represents chess figure 'sentry'
pub const SENTRY: Figure = 16;
/// JAILER represents chess figure 'jailer'
pub const JAILER: Figure = 17;

/// LANCER_MIN tells the lowest lancer
pub const LANCER_MIN: Figure = LANCERN;
/// LANCER_MAX tells the highest lancer
pub const LANCER_MAX: Figure = LANCERNW;

/// FIG_MIN tells the lowest non empty figure
pub const FIG_MIN: Figure = PAWN;
/// FIG_MAX tells the highest non empty figure
pub const FIG_MAX: Figure = JAILER;

/// FIGURE_FEN_SYMBOLS maps a figure to its fen symbol
pub const FIGURE_FEN_SYMBOLS: [&str; 18] = [
    ".", "p", "n", "b", "r", "q", "k", "l", "ln", "lne", "le", "lse", "ls", "lsw", "lw", "lnw",
    "s", "j",
];

/// FIGURE_SAN_LETTERS maps a figure to its san letter
pub const FIGURE_SAN_LETTERS: [&str; 18] = [
    ".", "P", "N", "B", "R", "Q", "K", "L", "L", "L", "L", "L", "L", "L", "L", "L", "S", "J",
];

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

/// EMPTY_ATTACK_TABLE defines an empty attack table, useful for initializing attack tables
pub const EMPTY_ATTACK_TABLE: AttackTable = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

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
