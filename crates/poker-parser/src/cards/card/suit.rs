#[derive(Debug, PartialEq)]
pub enum Suit {
    Hearts,
    Spades,
    Dimonds,
    Clubs
}

pub fn parse(card: &str) -> Suit {
    match &card[1..2] {
        "H" => Suit::Hearts,
        "D" => Suit::Dimonds,
        "C" => Suit::Clubs,
        _ => Suit::Spades,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parsers_h_to_hearts() {
        assert_eq!(parse("2H"), Suit::Hearts);
    }

    #[test]
    fn it_parsers_d_to_dimonds() {
        assert_eq!(parse("2D"), Suit::Dimonds);
    }

    #[test]
    fn it_parsers_s_to_spades() {
        assert_eq!(parse("2S"), Suit::Spades);
    }

    #[test]
    fn it_parsers_c_to_clubs() {
        assert_eq!(parse("2C"), Suit::Clubs);
    }

    #[test]
    fn it_parsers_any_other_letter_to_spades() {
        assert_eq!(parse("2X"), Suit::Spades);
    }

    #[test]
    fn it_works_with_all_numbers() {
        assert_eq!(parse("3H"), Suit::Hearts);
        assert_eq!(parse("4H"), Suit::Hearts);
        assert_eq!(parse("5H"), Suit::Hearts);
    }
}
