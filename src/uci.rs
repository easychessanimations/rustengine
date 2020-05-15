use crate::bitboard::*;
use crate::constants::*;
use crate::lineargame::*;
use crate::piece::*;
use crate::square::*;
//use crate::state::*;

use std::io::{self, BufRead};

pub struct Uci {
    pub engine_name: String,
    pub engine_author: String,
    pub linear_game: LinearGame,
}

pub fn create_default_uci() -> Uci {
    let mut uci = Uci {
        engine_name: "rustengine".to_string(),
        engine_author: "easychessanimations".to_string(),
        linear_game: LinearGame::new(),
    };
    uci.linear_game.init(DEFAULT_VARIANT);
    uci
}

pub fn demo() {
    let x: Bitboard = 0xffff00000000ffff;

    println!("{}", x.pretty_print_string());

    let sq: Square = rank_file(RANK_3, FILE_D);

    println!("square {} file {} rank {}", sq.uci(), sq.file(), sq.rank());

    let fig: Figure = LANCERNE;

    println!("\nfigure {} symbol {}", fig, fig.symbol());

    let p: Piece = color_figure(WHITE, LANCERNE);

    println!(
        "\npiece {} fen symbol {} san symbol {} uci symbol {} san letter {}",
        p,
        p.fen_symbol(),
        p.san_symbol(),
        p.uci_symbol(),
        p.san_letter()
    );

    let mut bb: Bitboard = sq.bitboard() | SQUARE_G6.bitboard();

    loop {
        println!("\n{}", bb.pretty_print_string());

        let (sq, ok) = bb.pop_square();

        if ok {
            println!("{}", sq.uci());
        } else {
            println!("no square could be popped\n");
            break;
        }
    }

    println!(
        "{}",
        jump_attack(SQUARE_E4, &KNIGHT_DELTAS, SQUARE_F6.bitboard()).pretty_print_string()
    );

    println!(
        "{}",
        sliding_attack(SQUARE_E4, &QUEEN_DELTAS, SQUARE_G6.bitboard()).pretty_print_string()
    );

    println!("{}", BISHOP_ATTACK[SQUARE_C7].pretty_print_string());

    println!("{}", KING_AREA[SQUARE_G8].pretty_print_string());
}

pub fn enum_occup_demo() {
    let occup = BISHOP_ATTACK[SQUARE_C7];

    let mut mask: usize = 0;

    loop {
        if mask < occup.variation_count() {
            println!(
                "{}\n{}",
                mask,
                translate_mask_to_occupancy(mask, occup).pretty_print_string()
            );
            mask += 1;
        } else {
            break;
        }
    }
}

pub fn mobility_demo() {
    println!(
        "{}",
        queen_mobility(
            SQUARE_E4,
            MoveGenMode::All,
            SQUARE_G6.bitboard() | SQUARE_C4.bitboard(),
            SQUARE_D5.bitboard() | SQUARE_E7.bitboard()
        )
        .pretty_print_string()
    )
}

pub fn magic_space() {
    let tb = total_magic_space(BISHOP_MAGICS);

    let sb = tb * std::mem::size_of::<Bitboard>();

    println!("total bishop magic space {} bytes", sb);

    let tr = total_magic_space(ROOK_MAGICS);

    let sr = tr * std::mem::size_of::<Bitboard>();

    println!("total rook magic space {} bytes", sr);

    println!(
        "\ngrand total magic space {} bytes = {:0.2} MiBs",
        sb + sr,
        (sb + sr) as f32 / 1e6
    );

    println!("magic units bishop {} rook {} total {}", tb, tr, tb + tr);
}

impl Uci {
    pub fn execute_uci_command(&self) {
        println!("id name {}", self.engine_name);
        println!("id author {}\n", self.engine_author);
        println!("uciok");
    }

    pub fn process_uci_command(&mut self, line: String) -> bool {
        let parts: Vec<&str> = line.split(" ").collect();

        let command = parts[0];

        if command == "quit" || command == "q" || command == "exit" || command == "x" {
            return false;
        }

        if command == "uci" {
            self.execute_uci_command();
        }

        if command == "i" {
            self.linear_game.print();
        }

        if command == "demo" {
            let mut arg = "";

            if parts.len() > 0 {
                arg = parts[0];
            }

            match arg {
                "occup" => enum_occup_demo(),
                "mob" => mobility_demo(),
                "space" => magic_space(),
                _ => demo(),
            }
        }

        if command == "bb" {
            self.linear_game.current().print_bitboards();
        }

        true
    }

    pub fn welcome(&self, build_info: &str) {
        println!(
            "{} bitboard multi variant uci chess analysis engine by {} [ {} ]",
            self.engine_name, self.engine_author, build_info
        );
    }

    pub fn uci_loop(&mut self) {
        let stdin = io::stdin();

        for line in stdin.lock().lines() {
            if !self.process_uci_command(line.unwrap()) {
                break;
            }
        }
    }
}
