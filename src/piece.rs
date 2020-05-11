/// Figure type represents a chess figure as an unsigned int
pub type Figure = usize;

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

/// FIGURE_FEN_SYMBOLS maps a figure to its fen symbol
pub const FIGURE_FEN_SYMBOLS: [&str; 18] = [
    ".", "p", "n", "b", "r", "q", "k", "l", "ln", "lne", "le", "lse", "ls", "lsw", "lw", "lnw",
    "s", "j",
];

/// FIGURE_SAN_LETTERS maps a figure to its san letter
pub const FIGURE_SAN_LETTERS: [&str; 18] = [
    ".", "P", "N", "B", "R", "Q", "K", "L", "L", "L", "L", "L", "L", "L", "L", "L", "S", "J",
];

/// FigureTrait adds methods to Figure
pub trait FigureTrait {
    /// returns the fen symbol for the figure ( lower case )
    fn symbol(self) -> &'static str;
}

impl FigureTrait for Figure {
    /// returns the fen symbol for the figure ( lower case )
    fn symbol(self) -> &'static str {
        FIGURE_FEN_SYMBOLS[self]
    }
}

/// Color type represents a chess color
pub type Color = usize;

/// BLACK represents black chess color
pub const BLACK: Color = 0;
/// WHITE represents white chess color
pub const WHITE: Color = 1;

/// Piece type represents a chess piece as an unsigned int
pub type Piece = usize;

/// PIECE_FEN_SYMBOLS maps a piece to its fen symbol
pub const PIECE_FEN_SYMBOLS: [&str; 36] = [
    ".", ".", "p", "P", "n", "N", "b", "B", "r", "R", "q", "Q", "k", "K", "l", "L", "ln", "Ln",
    "lne", "Lne", "le", "Le", "lse", "Lse", "ls", "Ls", "lsw", "Lsw", "lw", "Lw", "lnw", "Lnw",
    "s", "S", "j", "J",
];

/// returns a piece from color and figure
pub fn color_figure(col: Color, fig: Figure) -> Piece {
    (2 * fig) + col
}

/// PieceTrait adds methods to Piece
pub trait PieceTrait {
    /// returns the color of the piece
    fn color(self) -> Color;
    /// returns the figure of the piece
    fn figure(self) -> Figure;
    /// returns the fen symbol for the piece
    fn fen_symbol(self) -> &'static str;
    /// returns the san symbol for the piece ( capital piece letter )
    fn san_symbol(self) -> &'static str;
    /// returns the uci symbol of the piece ( lower case )
    fn uci_symbol(self) -> &'static str;
    /// returns the san letter of the piece ( upper case )
    fn san_letter(self) -> &'static str;
}

impl PieceTrait for Piece {
    /// returns the color of the piece
    fn color(self) -> Color {
        return if self & 1 == 0 { BLACK } else { WHITE };
    }
    /// returns the figure of the piece
    fn figure(self) -> Figure {
        self >> 1
    }
    /// returns the fen symbol for the piece
    fn fen_symbol(self) -> &'static str {
        PIECE_FEN_SYMBOLS[self]
    }
    /// returns the san symbol for the piece ( capital piece letter )
    fn san_symbol(self) -> &'static str {
        return color_figure(WHITE, self.figure()).fen_symbol();
    }
    /// returns the uci symbol of the piece ( lower case )
    fn uci_symbol(self) -> &'static str {
        return self.figure().symbol();
    }
    /// returns the san letter of the piece ( upper case )
    fn san_letter(self) -> &'static str {
        FIGURE_SAN_LETTERS[self.figure()]
    }
}
