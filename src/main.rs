use rustenginelib::bitboard::*;
use rustenginelib::piece::*;
use rustenginelib::square::*;

fn main() {
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

    let mut bb: Bitboard = sq.bitboard() | SQUARE_G6.bitboard() | 0xf00;

    loop {
        println!("\n{}", bb.pretty_print_string());

        let (sq, ok) = bb.pop();

        if ok {
            println!("\n{}", sq.uci());
        } else {
            println!("no square could be popped");
            break;
        }
    }
}
