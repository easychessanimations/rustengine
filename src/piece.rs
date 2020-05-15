use crate::constants::*;

/// Figure type represents a chess figure as an unsigned int
pub type Figure = usize;

/// FigureTrait adds methods to Figure
pub trait FigureTrait {
    /// returns the fen symbol for the figure ( lower case )
    fn symbol(self) -> &'static str;
    /// returns the base figure of the figure, same as figure except for lancers, where it is LANCER
    fn base_figure(self) -> Figure;
    /// return the lancer direction of the figure provided that it is a lancer
    fn lancer_direction(self) -> usize;
}

impl FigureTrait for Figure {
    /// returns the fen symbol for the figure ( lower case )
    fn symbol(self) -> &'static str {
        FIGURE_FEN_SYMBOLS[self]
    }
    /// returns the base figure of the figure, same as figure except for lancers, where it is LANCER
    fn base_figure(self) -> Figure {
        if self < LANCER_MIN || self > LANCER_MAX {
            return self;
        }
        LANCER
    }
    /// return the lancer direction of the figure provided that it is a lancer
    fn lancer_direction(self) -> usize {
        self - LANCER_MIN
    }
}

/// Color type represents a chess color
pub type Color = usize;

/// ColorTrait defines methods for Color
pub trait ColorTrait {
    /// returns the color fen string
    fn turn_fen(self) -> String;
    /// returns inverse of color
    fn inverse(self) -> Color;
}

/// ColorTrait implementation
impl ColorTrait for Color {
    fn turn_fen(self) -> String {
        (if self == WHITE { "w" } else { "b" }).to_string()
    }
    /// returns inverse of color
    fn inverse(self) -> Color {
        WHITE - self
    }
}

/// Piece type represents a chess piece as an unsigned int
pub type Piece = usize;

/// returns the piece for the fen symbol
pub fn fen_symbol_to_piece(letter: &str) -> Piece {
    match letter {
        "p" => color_figure(BLACK, PAWN),
        "P" => color_figure(WHITE, PAWN),
        "n" => color_figure(BLACK, KNIGHT),
        "N" => color_figure(WHITE, KNIGHT),
        "b" => color_figure(BLACK, BISHOP),
        "B" => color_figure(WHITE, BISHOP),
        "r" => color_figure(BLACK, ROOK),
        "R" => color_figure(WHITE, ROOK),
        "q" => color_figure(BLACK, QUEEN),
        "Q" => color_figure(WHITE, QUEEN),
        "k" => color_figure(BLACK, KING),
        "K" => color_figure(WHITE, KING),
        "s" => color_figure(BLACK, SENTRY),
        "S" => color_figure(WHITE, SENTRY),
        "j" => color_figure(BLACK, JAILER),
        "J" => color_figure(WHITE, JAILER),
        _ => NO_PIECE,
    }
}

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
