//use crate::bitboard::*;
use crate::constants::*;
//use crate::piece::*;
//use crate::square::*;
use crate::state::*;

/// LinearGame represents a single variantion chess game
pub struct LinearGame {
    pub states: Vec<State>,
    pub state_ptr: usize,
}

/// LinearGame implementation
impl LinearGame {
    /// returns a new empty LinearGame
    pub fn new() -> LinearGame {
        LinearGame {
            states: vec![State::new(); MAX_STATES],
            state_ptr: 0,
        }
    }

    /// current returns the current state of the game
    pub fn current(&mut self) -> &mut State {
        &mut self.states[self.state_ptr]
    }

    /// returns the game as pretty printable string
    pub fn pretty_print_string(&mut self) -> String {
        self.current().pretty_print_string()
    }

    /// prints the game
    pub fn print(&mut self) {
        println!("{}", self.pretty_print_string())
    }

    /// initializes game to variant
    pub fn init(&mut self, variant: Variant) {
        self.state_ptr = 0;
        self.current().init(variant);
    }
}
