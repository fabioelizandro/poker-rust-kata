mod cards;

pub use self::cards::Suit;
pub use self::cards::Card;

#[derive(Debug, PartialEq)]
pub struct PokerPlayer {
    pub id: String,
    pub cards: Vec<Card>
}

pub fn parse(raw_player_and_cards: &str) -> PokerPlayer {
    let splited_id_and_cards: Vec<&str> = raw_player_and_cards
        .split(": ")
        .collect();

    let id = splited_id_and_cards[0];
    let cards = splited_id_and_cards[1];

    PokerPlayer {
        id: id.to_string(),
        cards: cards::parse(cards)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parsers_string_into_poker_player() {
        let raw_player_and_cards = "Player1: 2H 3D 5S 9C KD";

        assert_eq!(parse(raw_player_and_cards), PokerPlayer {
            id: "Player1".to_string(),
            cards: vec![
                Card { number: 2, suit: Suit::Hearts },
                Card { number: 3, suit: Suit::Dimonds },
                Card { number: 5, suit: Suit::Spades },
                Card { number: 9, suit: Suit::Clubs },
                Card { number: 13, suit: Suit::Dimonds }
            ]
        })
    }
}
