use rustenginelib::bitboard::*;
use rustenginelib::piece::*;
use rustenginelib::square::*;

fn main() {
    let x: Bitboard = 0xffff00000000ffff;

    println!("{}", x.pretty_print_string());

    let sq: Square = rank_file(RANK_3, FILE_D);

    println!("square {} file {} rank {}", sq.uci(), sq.file(), sq.rank());

    let fig: Figure = LANCERNE;

    println!("\nfigure {} fen symbol {}", fig, fig.fen_symbol())
}
