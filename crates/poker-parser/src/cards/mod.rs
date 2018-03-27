pub mod card;

pub use self::card::Suit;
pub use self::card::Card;

pub fn parse(cards: &str) -> Vec<Card> {
    cards
        .split(' ')
        .map(card::parse)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parsers_string_into_a_card_list() {
        assert_eq!(parse("2H 4S AD"), vec![
            Card {
                number: 2,
                suit: Suit::Hearts
            },

            Card {
                number: 4,
                suit: Suit::Spades
            },

            Card {
                number: 14,
                suit: Suit::Dimonds
            }
        ]);
    }
}
