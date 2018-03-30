mod player;

pub use self::player::Suit;
pub use self::player::Card;
pub use self::player::PokerPlayer;

#[derive(Debug, PartialEq)]
pub struct PokerGame {
    pub players: Vec<PokerPlayer>,
}

pub fn parse(raw_players: &str) -> PokerGame {
    let players: Vec<PokerPlayer> = raw_players
        .split("  ")
        .map(player::parse)
        .collect();

    PokerGame {
        players: players
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parsers_string_into_poker_game() {
        let raw_players = "Player1: 2H 3D 5S 9C KD  Player2: 2C 3H 4S 8C AH";

        assert_eq!(parse(raw_players), PokerGame {
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
        })
    }
}
