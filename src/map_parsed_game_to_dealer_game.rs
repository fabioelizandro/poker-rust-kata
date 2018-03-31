extern crate poker_parser;
extern crate poker_dealer;

fn map_card_suit(suit: poker_parser::Suit) -> poker_dealer::Suit {
    match suit {
        poker_parser::Suit::Hearts => poker_dealer::Suit::Hearts,
        poker_parser::Suit::Spades => poker_dealer::Suit::Spades,
        poker_parser::Suit::Clubs => poker_dealer::Suit::Clubs,
        poker_parser::Suit::Dimonds => poker_dealer::Suit::Dimonds
    }
}

fn map_card(card: poker_parser::Card) -> poker_dealer::Card {
    poker_dealer::Card {
        number: card.number,
        suit: map_card_suit(card.suit)
    }
}

fn map_player(player: poker_parser::PokerPlayer) -> poker_dealer::PokerPlayer {
    poker_dealer::PokerPlayer {
        id: player.id,
        cards: player.cards.into_iter().map(map_card).collect()
    }
}

pub fn map(game: poker_parser::PokerGame) -> poker_dealer::PokerGame {
    poker_dealer::PokerGame {
        players: game.players.into_iter().map(map_player).collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

   #[test]
    fn it_map_parsed_game_into_dealer_game() {
        let parsed_game = poker_parser::PokerGame {
            players: vec![
                poker_parser::PokerPlayer {
                    id: "Player1".to_string(),
                    cards: vec![
                        poker_parser::Card { number: 2, suit: poker_parser::Suit::Hearts },
                        poker_parser::Card { number: 3, suit: poker_parser::Suit::Dimonds },
                        poker_parser::Card { number: 5, suit: poker_parser::Suit::Spades },
                        poker_parser::Card { number: 9, suit: poker_parser::Suit::Clubs },
                        poker_parser::Card { number: 13, suit: poker_parser::Suit::Dimonds }
                    ]
                },

                poker_parser::PokerPlayer {
                    id: "Player2".to_string(),
                    cards: vec![
                        poker_parser::Card { number: 2, suit: poker_parser::Suit::Clubs },
                        poker_parser::Card { number: 3, suit: poker_parser::Suit::Hearts },
                        poker_parser::Card { number: 4, suit: poker_parser::Suit::Spades },
                        poker_parser::Card { number: 8, suit: poker_parser::Suit::Clubs },
                        poker_parser::Card { number: 14, suit: poker_parser::Suit::Hearts }
                    ]
                }
            ]
        };

        assert_eq!(map(parsed_game), poker_dealer::PokerGame {
            players: vec![
                poker_dealer::PokerPlayer {
                    id: "Player1".to_string(),
                    cards: vec![
                        poker_dealer::Card { number: 2, suit: poker_dealer::Suit::Hearts },
                        poker_dealer::Card { number: 3, suit: poker_dealer::Suit::Dimonds },
                        poker_dealer::Card { number: 5, suit: poker_dealer::Suit::Spades },
                        poker_dealer::Card { number: 9, suit: poker_dealer::Suit::Clubs },
                        poker_dealer::Card { number: 13, suit: poker_dealer::Suit::Dimonds }
                    ]
                },

                poker_dealer::PokerPlayer {
                    id: "Player2".to_string(),
                    cards: vec![
                        poker_dealer::Card { number: 2, suit: poker_dealer::Suit::Clubs },
                        poker_dealer::Card { number: 3, suit: poker_dealer::Suit::Hearts },
                        poker_dealer::Card { number: 4, suit: poker_dealer::Suit::Spades },
                        poker_dealer::Card { number: 8, suit: poker_dealer::Suit::Clubs },
                        poker_dealer::Card { number: 14, suit: poker_dealer::Suit::Hearts }
                    ]
                }
            ]
        })
    }
}
