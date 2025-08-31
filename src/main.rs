use colored::Colorize;

fn main() {
    let pawns = "         ";
    let pieces = "         ";

    println!("{}", pawns.on_blue());
    println!("{}", pieces.white());

    for j in 0..8 {
        for i in 0..8 {
            match (i + j) % 2 == 0 {
                true => {
                    print!("{}", "  ".on_white());
                }
                false => {
                    print!("{}", "  ".on_black());
                }
            }
        }
        println!();
    }
}
