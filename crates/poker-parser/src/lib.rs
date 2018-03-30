mod game;

pub use self::game::Suit;
pub use self::game::Card;
pub use self::game::PokerPlayer;
pub use self::game::PokerGame;

pub fn parse(raw_games: &str) -> Vec<PokerGame>  {
    raw_games
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(game::parse)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

   #[test]
    fn it_parsers_string_into_poker_game() {
        let raw_games = "
        Player1: 2H 3D 5S 9C KD  Player2: 2C 3H 4S 8C AH
        Player1: 2H 3D 5S 9C AD  Player2: 2C 3H 4S 8C KH
        ";

        assert_eq!(parse(raw_games), vec![
            PokerGame {
                players: vec![
                    PokerPlayer {
                        id: "Player1".to_string(),
                        cards: vec![
                            Card { number: 2, suit: Suit::Hearts },
                            Card { number: 3, suit: Suit::Dimonds },
                            Card { number: 5, suit: Suit::Spades },
                            Card { number: 9, suit: Suit::Clubs },
                            Card { number: 13, suit: Suit::Dimonds }
                        ]
                    },

                    PokerPlayer {
                        id: "Player2".to_string(),
                        cards: vec![
                            Card { number: 2, suit: Suit::Clubs },
                            Card { number: 3, suit: Suit::Hearts },
                            Card { number: 4, suit: Suit::Spades },
                            Card { number: 8, suit: Suit::Clubs },
                            Card { number: 14, suit: Suit::Hearts }
                        ]
                    }
                ]
            },

            PokerGame {
                players: vec![
                    PokerPlayer {
                        id: "Player1".to_string(),
                        cards: vec![
                            Card { number: 2, suit: Suit::Hearts },
                            Card { number: 3, suit: Suit::Dimonds },
                            Card { number: 5, suit: Suit::Spades },
                            Card { number: 9, suit: Suit::Clubs },
                            Card { number: 14, suit: Suit::Dimonds }
                        ]
                    },

                    PokerPlayer {
                        id: "Player2".to_string(),
                        cards: vec![
                            Card { number: 2, suit: Suit::Clubs },
                            Card { number: 3, suit: Suit::Hearts },
                            Card { number: 4, suit: Suit::Spades },
                            Card { number: 8, suit: Suit::Clubs },
                            Card { number: 13, suit: Suit::Hearts }
                        ]
                    }
                ]
            }
        ])
    }
}
