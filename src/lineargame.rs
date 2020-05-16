//use crate::bitboard::*;
use crate::constants::*;
//use crate::piece::*;
use crate::square::*;
use crate::state::*;

use std::time::Instant;

/// LinearGame represents a single variantion chess game
pub struct LinearGame {
    pub states: Vec<State>,
    pub state_ptr: usize,
    pub nodes: usize,
}

/// LinearGame implementation
impl LinearGame {
    /// returns a new empty LinearGame
    pub fn new() -> LinearGame {
        LinearGame {
            states: vec![State::new(); MAX_STATES],
            state_ptr: 0,
            nodes: 0,
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

    /// pushes a move
    pub fn push(&mut self, mv: Move) {
        self.state_ptr += 1;
        self.states[self.state_ptr] = self.states[self.state_ptr - 1].clone();
        self.current().make_move(mv);
    }

    /// pops a state
    pub fn pop(&mut self) {
        if self.state_ptr <= 0 {
            return;
        }
        self.state_ptr -= 1;
    }

    /// pushes a move by index in state move buff
    pub fn push_by_index(&mut self, index: usize) -> bool {
        if index < self.current().move_buff.len() {
            let mv = self.current().move_buff[index].mv;
            self.push(mv);
            return true;
        }
        false
    }

    pub fn perft_rec(&mut self, depth: usize) {
        self.nodes += 1;
        if depth == 0 {
            return;
        }
        let moves = self.current().generate_pseudo_legal_moves(MoveGenMode::All);
        for mv in moves.iter() {
            self.push(*mv);
            self.perft_rec(depth - 1);
            self.pop();
        }
    }

    pub fn perft(&mut self, depth: usize) -> (usize, f32, f32) {
        self.nodes = 0;
        let start = Instant::now();
        self.perft_rec(depth);
        let duration = start.elapsed();
        let secs = ((duration.as_secs() as f32) * 1e9 + (duration.subsec_nanos() as f32)) / 1e9;
        (self.nodes, secs, (self.nodes as f32) / secs / 1000.0)
    }
}
