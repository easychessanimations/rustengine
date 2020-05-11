pub trait Bitboard {
    fn pretty_print_string(&self) -> String;
}

impl Bitboard for u64 {
    fn pretty_print_string(&self) -> String {
        let mut bb = *self;
        let mut buff = "".to_string();
        let mut bits = 0;
        loop {
            if bits % 8 == 0 {
                buff += "*"
            }
            if bb & 1 == 1 {
                buff += "1"
            } else {
                buff += "0"
            }
            if bits % 8 == 7 {
                buff += "*\n"
            }
            bb = bb >> 1;
            bits = bits + 1;
            if bits == 64 {
                break;
            }
        }
        format! {"bitboard {:#016x}\n**********\n{}**********\n", &self, buff}
    }
}
