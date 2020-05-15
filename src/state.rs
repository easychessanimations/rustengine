use crate::bitboard::*;
use crate::constants::*;
use crate::piece::*;
use crate::square::*;

/// State records the state of a chess game
#[derive(Clone)]
pub struct State {
    variant: Variant,
    rep: [Piece; BOARD_AREA],
    turn: Color,
    by_color: [[Bitboard; FIGURE_ARRAY_SIZE]; 2],
}

/// Variant type records the index of the variant
pub type Variant = usize;

/// VariantInfo records variant information
pub struct VariantInfo {
    pub start_fen: &'static str,
    pub display_name: &'static str,
}

/// State implementation
impl State {
    /// parses piece placement
    pub fn parse_piece_placement(fen: &str) -> [Piece; BOARD_AREA] {
        let mut rep = EMPTY_REP;
        let mut rank: Rank = 0;
        let mut file: File = 0;
        let mut lancer_color = 0;
        let mut lancer_has_north = false;
        let mut lancer_index = 0;
        for i in 0..fen.len() {
            let c = &fen[i..i + 1];
            if file > LAST_FILE && c != "/" && c != " " {
                panic!("invalid piece placement file");
            }
            let mut examine_c = true;
            let sq: Square = (LAST_RANK - rank) * NUM_FILES + file;
            if c == "l" {
                lancer_color = BLACK;
                lancer_index = 1;
            } else if c == "L" {
                lancer_color = WHITE;
                lancer_index = 1;
            } else if lancer_index == 1 {
                if c == "n" {
                    lancer_has_north = true;
                    lancer_index = 2;
                } else if c == "s" {
                    lancer_has_north = false;
                    lancer_index = 2;
                } else if c == "e" {
                    rep[sq] = color_figure(lancer_color, LANCERE);
                    file += 1;
                    lancer_index = 0;
                    examine_c = false;
                } else if c == "w" {
                    rep[sq] = color_figure(lancer_color, LANCERW);
                    file += 1;
                    lancer_index = 0;
                    examine_c = false;
                } else {
                    panic!("invalid lancer")
                }
            } else if lancer_index == 2 {
                if c == "e" {
                    if lancer_has_north {
                        rep[sq] = color_figure(lancer_color, LANCERNE);
                        file += 1;
                        lancer_index = 0;
                        examine_c = false;
                    } else {
                        rep[sq] = color_figure(lancer_color, LANCERSE);
                        file += 1;
                        lancer_index = 0;
                        examine_c = false;
                    }
                } else if c == "w" {
                    if lancer_has_north {
                        rep[sq] = color_figure(lancer_color, LANCERNW);
                        file += 1;
                        lancer_index = 0;
                        examine_c = false;
                    } else {
                        rep[sq] = color_figure(lancer_color, LANCERSW);
                        file += 1;
                        lancer_index = 0;
                        examine_c = false;
                    }
                } else {
                    if lancer_has_north {
                        rep[sq] = color_figure(lancer_color, LANCERN);
                        file += 1;
                        lancer_index = 0;
                    } else {
                        rep[sq] = color_figure(lancer_color, LANCERS);
                        file += 1;
                        lancer_index = 0;
                    }
                }
            }
            if lancer_index == 0 && examine_c {
                if c == " " {
                    return rep;
                } else if c == "/" {
                    file = 0;
                    rank += 1;
                } else if c >= "1" && c <= "8" {
                    for _ in 0..c.parse().expect("should not happen") {
                        if file > LAST_FILE {
                            panic!("invalid piece placement file");
                        }
                        rep[(LAST_RANK - rank) * NUM_FILES + file] = NO_PIECE;
                        file += 1;
                    }
                } else {
                    let p = fen_symbol_to_piece(c);
                    if p != NO_PIECE {
                        rep[sq] = p;
                        file += 1;
                    } else {
                        panic!("invalid fen symbol")
                    }
                }
            }
        }
        rep
    }

    /// creates a new empty State
    pub fn new() -> State {
        State {
            variant: DEFAULT_VARIANT,
            rep: EMPTY_REP,
            turn: WHITE,
            by_color: [EMTPY_FIGURE_BITBOARDS, EMTPY_FIGURE_BITBOARDS],
        }
    }

    /// sets state from fen
    pub fn set_from_fen(&mut self, fen: &str) {
        let parts: Vec<&str> = fen.split(" ").collect();

        let l = parts.len();

        if l != 4 && l != 6 && l != 7 {
            panic!("invalid number of fen fields {}", l);
        }

        self.rep = State::parse_piece_placement(parts[0]);

        match parts[1] {
            "w" => self.turn = WHITE,
            "b" => self.turn = BLACK,
            _ => panic!("invalid turn {}", parts[1]),
        }
    }

    /// initializes state to variant
    pub fn init(&mut self, variant: Variant) {
        self.variant = variant;
        self.set_from_fen(VARIANT_INFOS[self.variant].start_fen);
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
        buff = format!(
            "{}\nvariant start fen : {}\n",
            buff,
            self.variant_start_fen()
        );
        buff
    }

    /// initialize from variant
    pub fn init_variant(&self) {}

    /// returns the start fen for the variant of the state
    pub fn variant_start_fen(&self) -> &str {
        VARIANT_INFOS[self.variant].start_fen
    }
}
