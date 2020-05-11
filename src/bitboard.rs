pub trait Bitboard {
    fn pretty_print_string(&self) -> String;
}

impl Bitboard for u64 {
    fn pretty_print_string(&self) -> String {
        format! {"{:#016x}", &self}
    }
}
