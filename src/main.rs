use rustenginelib::uci::*;

fn main() {
    let mut uci = create_default_uci();

    uci.welcome("native build");

    uci.linear_game.print();

    uci.uci_loop();
}
