mod suit;
mod number;

pub use self::suit::Suit;

#[derive(Debug, PartialEq)]
pub struct Card {
    number: u8,
    pub suit: Suit
}

pub fn parse(card: &str) -> Card {
    Card {
        number: number::parse(card),
        suit: suit::parse(card)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parsers_string_to_card() {
        assert_eq!(parse("2H"), Card { number: 2, suit: Suit::Hearts });
        assert_eq!(parse("7S"), Card { number: 7, suit: Suit::Spades });
        assert_eq!(parse("AD"), Card { number: 14, suit: Suit::Dimonds });
    }
}
