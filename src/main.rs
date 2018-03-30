extern crate poker_parser;

fn main() {
    let input = "
    Player1: 2H 3D 5S 9C KD  Player2: 2C 3H 4S 8C AH
    Player1: 2H 3D 5S 9C AD  Player2: 2C 3H 4S 8C KH
    ";

    println!("{:?}", poker_parser::parse(input));
}
