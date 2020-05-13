use rustenginelib::bitboard::*;
use rustenginelib::piece::*;
use rustenginelib::square::*;

fn demo() {
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

fn enum_occup_demo() {
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

fn magic_space() {
    let sb = total_magic_space(BISHOP_MAGICS) * std::mem::size_of::<Bitboard>();
    println!("total bishop magic space {} bytes", sb);

    let sr = total_magic_space(ROOK_MAGICS) * std::mem::size_of::<Bitboard>();

    println!("total rook magic space {} bytes", sr);

    println!(
        "\ngrand total magic space {} bytes = {:0.2} MiBs",
        sb + sr,
        (sb + sr) as f32 / 1e6
    );
}

fn main() {
    println!("\nhi rustengine\n");

    if false {
        demo();
    }

    if false {
        enum_occup_demo();
    }

    if false {
        find_and_log_magics();
    }

    if true {
        magic_space();
    }

    println!("\nbye rustengine\n");
}
