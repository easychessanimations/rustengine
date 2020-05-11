use rustenginelib::bitboard::Bitboard;
use rustenginelib::square::*;

fn main() {
    let x: u64 = 0xffff00000000ffff;
    let sq: usize = SQUARE_G6;

    println!(
        "{} sq {} file {} rank {}",
        x.pretty_print_string(),
        sq.uci(),
        sq.file(),
        sq.rank()
    )
}
