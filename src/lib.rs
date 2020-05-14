pub mod bitboard;
pub mod constants;
pub mod lineargame;
pub mod piece;
pub mod square;
pub mod state;
pub mod uci;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
