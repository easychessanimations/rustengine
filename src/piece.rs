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
    ".", "p", "n", "p", "r", "q", "k", "l", "ln", "lne", "le", "lse", "ls", "lsw", "lw", "lnw",
    "s", "j",
];

/// FigureTrait adds methods to Figure
pub trait FigureTrait {
    /// returns the fen symbol for the figure ( lower case )
    fn fen_symbol(self) -> &'static str;
}

impl FigureTrait for Figure {
    /// returns the fen symbol for the figure ( lower case )
    fn fen_symbol(self) -> &'static str {
        FIGURE_FEN_SYMBOLS[self]
    }
}
