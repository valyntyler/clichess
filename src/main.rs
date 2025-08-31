use clichess::board::Board;
use colored::Colorize;

fn main() {
    let pawns = "         ";
    let pieces = "         ";

    println!("{}", pawns.on_blue());
    println!("{}", pieces.white());

    println!();
    println!("{}", Board::new());
}
