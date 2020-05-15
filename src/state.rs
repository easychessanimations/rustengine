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
    ep_square: Square,
    halfmove_clock: usize,
    fullmove_number: usize,
    has_disabled_move: bool,
    disable_from_sq: Square,
    disable_to_sq: Square,
    by_color: [[Bitboard; FIGURE_ARRAY_SIZE]; 2],
    castling_rights: [ColorCastlingRights; 2],
}

/// CastlingRight represents a castling right
#[derive(Clone)]
pub struct CastlingRight {
    pub can_castle: bool,
}

/// ColorCastlingRights represents castling rights for a color
#[derive(Clone)]
pub struct ColorCastlingRights {
    pub rights: [CastlingRight; 2],
}

/// Variant type records the index of the variant
pub type Variant = usize;

trait VariantTrait {
    /// returns name of variant
    fn string(self) -> String;
}

impl VariantTrait for Variant {
    /// returns name of variant
    fn string(self) -> String {
        let name = match self {
            VARIANT_STANDARD => "Standard",
            VARIANT_EIGHTPIECE => "Eightpiece",
            VARIANT_ATOMIC => "Atomic",
            _ => "Unknownvariant",
        };
        name.to_string()
    }
}

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
            ep_square: SQUARE_A1,
            halfmove_clock: 0,
            fullmove_number: 1,
            has_disabled_move: false,
            disable_from_sq: SQUARE_A1,
            disable_to_sq: SQUARE_A1,
            by_color: [EMTPY_FIGURE_BITBOARDS, EMTPY_FIGURE_BITBOARDS],
            castling_rights: [EMPTY_COLOR_CASTLING_RIGHTS, EMPTY_COLOR_CASTLING_RIGHTS],
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

        self.castling_rights = [EMPTY_COLOR_CASTLING_RIGHTS, EMPTY_COLOR_CASTLING_RIGHTS];

        if parts[2] == "-" {
            // no castling rights
        } else {
            for i in 0..parts[2].len() {
                let r = &parts[2][i..i + 1];
                match r {
                    "K" => self.castling_rights[WHITE].rights[KING_SIDE].can_castle = true,
                    "Q" => self.castling_rights[WHITE].rights[QUEEN_SIDE].can_castle = true,
                    "k" => self.castling_rights[BLACK].rights[KING_SIDE].can_castle = true,
                    "q" => self.castling_rights[BLACK].rights[QUEEN_SIDE].can_castle = true,
                    _ => panic!("invalid castling right {}", r),
                }
            }
        }

        self.ep_square = SQUARE_A1;

        if parts[3] != "-" {
            self.ep_square = Square::from_uci(parts[3].to_string()).0;
        }

        self.halfmove_clock = parts[4].parse().expect("invalid halfmove clock");
        self.fullmove_number = parts[5].parse().expect("invalid fullmove number");

        self.has_disabled_move = false;

        if self.variant == VARIANT_EIGHTPIECE {
            if parts.len() > 6 {
                if parts[6] != "-" {
                    if parts[6].len() != 4 {
                        panic!("invalid disabled mvoe {:?}", parts[6]);
                    }
                    let df = Square::from_uci(parts[6][0..2].to_string());
                    let dt = Square::from_uci(parts[6][2..4].to_string());
                    if df.1 && dt.1 {
                        self.disable_from_sq = df.0;
                        self.disable_to_sq = dt.0;
                    } else {
                        panic!("invalid disabled move {}", parts[6]);
                    }
                    self.has_disabled_move = true;
                }
            }
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

    /// reports the state as fen
    pub fn report_fen(&self) -> String {
        let mut buff = "".to_string();
        let mut acc = 0;
        for rank in 0..NUM_RANKS {
            for file in 0..NUM_FILES {
                let sq: Square = (LAST_RANK - rank) * NUM_FILES + file;
                let p = self.piece_at_square(sq);
                let mut should_flush = false;
                if p == NO_PIECE {
                    acc += 1;
                } else {
                    should_flush = true;
                    buff = format!("{}{}", buff, p.fen_symbol());
                }
                if acc > 0 && (should_flush || file == LAST_FILE) {
                    buff = format!("{}{}", buff, acc);
                    acc = 0;
                }
                if file == LAST_FILE && rank < LAST_RANK {
                    buff = format!("{}/", buff);
                }
            }
        }
        buff = format!("{} {}", buff, self.turn.turn_fen());
        let mut cfen = "".to_string();
        if self.castling_rights[WHITE].rights[KING_SIDE].can_castle {
            cfen = format!("{}{}", cfen, "K")
        }
        if self.castling_rights[WHITE].rights[QUEEN_SIDE].can_castle {
            cfen = format!("{}{}", cfen, "Q")
        }
        if self.castling_rights[BLACK].rights[KING_SIDE].can_castle {
            cfen = format!("{}{}", cfen, "k")
        }
        if self.castling_rights[BLACK].rights[QUEEN_SIDE].can_castle {
            cfen = format!("{}{}", cfen, "q")
        }
        if cfen == "" {
            cfen = "-".to_string();
        }
        let mut epfen = "-".to_string();
        if self.ep_square != SQUARE_A1 {
            epfen = self.ep_square.uci();
        }
        buff = format!(
            "{} {} {} {} {}",
            buff, cfen, epfen, self.halfmove_clock, self.fullmove_number
        );
        if self.variant == VARIANT_EIGHTPIECE {
            let mut dfen = "-".to_string();
            if self.has_disabled_move {
                dfen = format!("{}{}", self.disable_from_sq.uci(), self.disable_to_sq.uci());
            }
            buff = format!("{} {}", buff, dfen);
        }
        buff
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
            "{}\n\nvariant {} fen {}\n",
            buff,
            self.variant.string(),
            self.report_fen()
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
