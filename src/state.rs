use crate::bitboard::*;
use crate::constants::*;
use crate::piece::*;
use crate::square::*;

/// State records the state of a chess game
#[derive(Clone)]
pub struct State {
    rep: [Piece; BOARD_AREA],
    by_color: [[Bitboard; FIGURE_ARRAY_SIZE]; 2],
}

/// State implementation
impl State {
    /// creates a new empty State
    pub fn new() -> State {
        State {
            rep: EMPTY_REP,
            by_color: [EMTPY_FIGURE_BITBOARDS, EMTPY_FIGURE_BITBOARDS],
        }
    }

    /// sets the piece at a square
    pub fn set_piece_at_square(&mut self, sq: Square, p: Piece) {
        self.rep[sq] = p;
    }

    /// returns the piece at a square
    pub fn piece_at_square(&self, sq: Square) -> Piece {
        self.rep[sq]
    }

    /// returns the state as pretty printable string
    pub fn pretty_print_string(&self) -> String {
        let mut buff = "".to_string();
        for rank in 0..NUM_RANKS {
            for file in 0..NUM_FILES {
                let sq = rank_file(LAST_RANK - rank, file);
                let p = self.piece_at_square(sq);
                buff = format!("{}{:^3}", buff, p.fen_symbol());
                if file == LAST_FILE {
                    buff += "\n";
                }
            }
        }
        buff
    }
}
