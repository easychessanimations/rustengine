use rustenginelib::uci::*;

fn main() {
    let uci = create_default_uci();

    uci.welcome("native build");

    uci.uci_loop();
}
