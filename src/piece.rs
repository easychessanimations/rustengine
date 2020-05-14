use crate::constants::*;

/// Figure type represents a chess figure as an unsigned int
pub type Figure = usize;

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

/// Piece type represents a chess piece as an unsigned int
pub type Piece = usize;

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
