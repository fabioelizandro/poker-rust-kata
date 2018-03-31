extern crate poker_parser;
extern crate poker_dealer;

mod map_parsed_game_to_dealer_game;

fn main() {
    let input = "
    Player1: 2H 3D 5S 9C KD  Player2: 2C 3H 4S 8C AH
    Player1: 2H 3D 5S 9C AD  Player2: 2C 3H 4S 8C KH
    ";

    poker_parser::parse(input)
        .into_iter()
        .map(map_parsed_game_to_dealer_game::map)
        .for_each(poker_dealer::result)
}
