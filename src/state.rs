use crate::bitboard::*;
use crate::constants::*;
use crate::piece::*;
use crate::square::*;

/// MoveBuffItem stores a move with meta information
#[derive(Clone)]
pub struct MoveBuffItem {
    pub mv: Move,
    uci: String,
}

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
    by_figure: [[Bitboard; FIGURE_ARRAY_SIZE]; 2],
    by_color: [Bitboard; 2],
    castling_rights: [ColorCastlingRights; 2],
    pub move_buff: Vec<MoveBuffItem>,
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
    pub fn parse_piece_placement(&mut self, fen: &str) {
        self.by_figure = [EMTPY_FIGURE_BITBOARDS, EMTPY_FIGURE_BITBOARDS];
        self.by_color = [0, 0];
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
                    self.put(sq, color_figure(lancer_color, LANCERE));
                    file += 1;
                    lancer_index = 0;
                    examine_c = false;
                } else if c == "w" {
                    self.put(sq, color_figure(lancer_color, LANCERW));
                    file += 1;
                    lancer_index = 0;
                    examine_c = false;
                } else {
                    panic!("invalid lancer")
                }
            } else if lancer_index == 2 {
                if c == "e" {
                    if lancer_has_north {
                        self.put(sq, color_figure(lancer_color, LANCERNE));
                        file += 1;
                        lancer_index = 0;
                        examine_c = false;
                    } else {
                        self.put(sq, color_figure(lancer_color, LANCERSE));
                        file += 1;
                        lancer_index = 0;
                        examine_c = false;
                    }
                } else if c == "w" {
                    if lancer_has_north {
                        self.put(sq, color_figure(lancer_color, LANCERNW));
                        file += 1;
                        lancer_index = 0;
                        examine_c = false;
                    } else {
                        self.put(sq, color_figure(lancer_color, LANCERSW));
                        file += 1;
                        lancer_index = 0;
                        examine_c = false;
                    }
                } else {
                    if lancer_has_north {
                        self.put(sq, color_figure(lancer_color, LANCERN));
                        file += 1;
                        lancer_index = 0;
                    } else {
                        self.put(sq, color_figure(lancer_color, LANCERS));
                        file += 1;
                        lancer_index = 0;
                    }
                }
            }
            if lancer_index == 0 && examine_c {
                if c == " " {
                    return;
                } else if c == "/" {
                    file = 0;
                    rank += 1;
                } else if c >= "1" && c <= "8" {
                    for _ in 0..c.parse().expect("should not happen") {
                        if file > LAST_FILE {
                            panic!("invalid piece placement file");
                        }
                        self.remove((LAST_RANK - rank) * NUM_FILES + file);
                        file += 1;
                    }
                } else {
                    let p = fen_symbol_to_piece(c);
                    if p != NO_PIECE {
                        self.put(sq, p);
                        file += 1;
                    } else {
                        panic!("invalid fen symbol")
                    }
                }
            }
        }
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
            by_figure: [EMTPY_FIGURE_BITBOARDS, EMTPY_FIGURE_BITBOARDS],
            by_color: [0, 0],
            castling_rights: [EMPTY_COLOR_CASTLING_RIGHTS, EMPTY_COLOR_CASTLING_RIGHTS],
            move_buff: Vec::new(),
        }
    }

    /// sets state from fen
    pub fn set_from_fen(&mut self, fen: &str) {
        let parts: Vec<&str> = fen.split(" ").collect();

        let l = parts.len();

        if l != 4 && l != 6 && l != 7 {
            panic!("invalid number of fen fields {}", l);
        }

        self.parse_piece_placement(parts[0]);

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

    // puts a piece on a square
    pub fn put(&mut self, sq: Square, p: Piece) {
        if p == NO_PIECE {
            return;
        }
        self.rep[sq] = p;
        let bb = sq.bitboard();
        self.by_figure[p.color()][p.figure()] |= bb;
        self.by_color[p.color()] |= bb;
    }

    // removes the piece from a square
    pub fn remove(&mut self, sq: Square) {
        let p = self.piece_at_square(sq);
        if p == NO_PIECE {
            return;
        }
        self.rep[sq] = NO_PIECE;
        let bb = sq.bitboard();
        self.by_figure[p.color()][p.figure()] &= !bb;
        self.by_color[p.color()] &= !bb;
    }

    /// initializes state to variant
    pub fn init(&mut self, variant: Variant) {
        self.variant = variant;
        self.set_from_fen(VARIANT_INFOS[self.variant].start_fen);
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

    /// prints bitboards
    pub fn print_bitboards(&self) {
        for col in BLACK..WHITE + 1 {
            for fig in FIG_MIN..FIG_MAX {
                println!(
                    "{} {} {}",
                    col.turn_fen(),
                    fig.symbol(),
                    self.by_figure[col][fig].pretty_print_string()
                );
            }
        }
    }

    /// returns mobility of color figure at square
    pub fn color_figure_mobility_at_square(
        &self,
        sq: Square,
        gen_mode: MoveGenMode,
        col: Color,
        fig: Figure,
    ) -> Bitboard {
        match fig.base_figure() {
            KNIGHT => knight_mobility(
                sq,
                gen_mode,
                self.by_color[col],
                self.by_color[col.inverse()],
            ),
            BISHOP => bishop_mobility(
                sq,
                gen_mode,
                self.by_color[col],
                self.by_color[col.inverse()],
            ),
            SENTRY => bishop_mobility(
                sq,
                gen_mode,
                self.by_color[col],
                self.by_color[col.inverse()],
            ),
            ROOK => rook_mobility(
                sq,
                gen_mode,
                self.by_color[col],
                self.by_color[col.inverse()],
            ),
            JAILER => jailer_mobility(
                sq,
                gen_mode,
                self.by_color[col],
                self.by_color[col.inverse()],
            ),
            QUEEN => queen_mobility(
                sq,
                gen_mode,
                self.by_color[col],
                self.by_color[col.inverse()],
            ),
            LANCER => lancer_mobility(
                sq,
                gen_mode,
                self.by_color[col],
                self.by_color[col.inverse()],
                LANCER_ATTACKS[fig.lancer_direction()][sq],
            ),
            KING => king_mobility(
                sq,
                gen_mode,
                self.by_color[col],
                self.by_color[col.inverse()],
            ),
            _ => 0,
        }
    }

    /// generates pseudo legal moves for color
    pub fn generate_pseudo_legal_moves_for_color(
        &self,
        gen_mode: MoveGenMode,
        col: Color,
    ) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![0; 0];
        let mut bb = self.by_color[col];
        loop {
            let (sq, ok) = bb.pop_square();
            if ok {
                let p = self.piece_at_square(sq);
                let fig = p.figure();
                match fig {
                    PAWN => {
                        let pi: &PawnInfo = &PAWN_INFOS[col][sq];
                        let mut do_pawn_moves = |pushes, captures| {
                            if pushes {
                                for to_sq in pi.pushes.iter() {
                                    if self.piece_at_square(*to_sq) == NO_PIECE {
                                        moves.push(Move::ft(sq, *to_sq));
                                    }
                                }
                            }
                            if captures {
                                for to_sq in pi.captures.iter() {
                                    let cp: Piece = self.piece_at_square(*to_sq);
                                    if cp != NO_PIECE && cp.color() == col.inverse() {
                                        moves.push(Move::ft(sq, *to_sq));
                                    }
                                }
                            }
                        };
                        match gen_mode {
                            MoveGenMode::Violent => do_pawn_moves(false, true),
                            MoveGenMode::Quiet => do_pawn_moves(true, false),
                            MoveGenMode::All => do_pawn_moves(true, true),
                        }
                    }
                    _ => {
                        let mut mob =
                            self.color_figure_mobility_at_square(sq, gen_mode, col, p.figure());
                        loop {
                            let (to_sq, ok) = mob.pop_square();
                            if ok {
                                moves.push(Move::ft(sq, to_sq));
                            } else {
                                break;
                            }
                        }
                    }
                }
            } else {
                break;
            }
        }
        moves
    }

    /// returns the state as pretty printable string
    pub fn pretty_print_string(&mut self) -> String {
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
            "{}\nvariant {} fen {}\n",
            buff,
            self.variant.string(),
            self.report_fen()
        );
        format!("{}\n{}\n", buff, self.gen_move_buff())
    }

    /// generates moves with meta information
    pub fn gen_move_buff(&mut self) -> String {
        let moves = self.generate_pseudo_legal_moves_for_color(MoveGenMode::All, self.turn);
        let mut move_buff = "".to_string();
        self.move_buff = Vec::new();
        for i in 0..moves.len() {
            let mv = moves[i];
            self.move_buff.push(MoveBuffItem {
                mv: mv,
                uci: mv.uci(),
            });
        }
        self.move_buff.sort_by(|a, b| a.uci.cmp(&b.uci));
        for i in 0..self.move_buff.len() {
            let move_str = format!("{}. {}", i + 1, self.move_buff[i].uci);
            move_buff = format!("{}{:16}", move_buff, move_str);
            if i % 6 == 5 {
                move_buff = format!("{}\n", move_buff);
            }
        }
        move_buff
    }

    /// initialize from variant
    pub fn init_variant(&self) {}

    /// returns the start fen for the variant of the state
    pub fn variant_start_fen(&self) -> &str {
        VARIANT_INFOS[self.variant].start_fen
    }

    /// makes a move
    pub fn make_move(&mut self, mv: Move) {
        let from_sq = mv.from_sq();
        let to_sq = mv.to_sq();
        let fromp: Piece = self.piece_at_square(from_sq);
        //let top: Piece = self.piece_at_square(to_sq);
        self.remove(from_sq);
        self.put(to_sq, fromp);

        self.turn = self.turn.inverse();
    }
}
