pub fn parse(card: &str) -> u8 {
    let suits: &[_] = &['H', 'S', 'C', 'D'];
    let card_number = card.trim_matches(suits);

    match card_number {
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "10" => 10,
        "J" => 11,
        "Q" => 12,
        "K" => 13,
        _ => 14,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parsers_2_to_integer_2() {
        assert_eq!(parse("2H"), 2);
    }

    #[test]
    fn it_parsers_3_to_integer_3() {
        assert_eq!(parse("3H"), 3);
    }

    #[test]
    fn it_parsers_4_to_integer_4() {
        assert_eq!(parse("4H"), 4);
    }

    #[test]
    fn it_parsers_5_to_integer_5() {
        assert_eq!(parse("5H"), 5);
    }

    #[test]
    fn it_parsers_6_to_integer_6() {
        assert_eq!(parse("6H"), 6);
    }

    #[test]
    fn it_parsers_7_to_integer_7() {
        assert_eq!(parse("7H"), 7);
    }

    #[test]
    fn it_parsers_8_to_integer_8() {
        assert_eq!(parse("8H"), 8);
    }

    #[test]
    fn it_parsers_9_to_integer_9() {
        assert_eq!(parse("9H"), 9);
    }

    #[test]
    fn it_parsers_10_to_integer_10() {
        assert_eq!(parse("10H"), 10);
    }

    #[test]
    fn it_parsers_j_to_integer_11() {
        assert_eq!(parse("JH"), 11);
    }

    #[test]
    fn it_parsers_q_to_integer_12() {
        assert_eq!(parse("QH"), 12);
    }

    #[test]
    fn it_parsers_k_to_integer_13() {
        assert_eq!(parse("KH"), 13);
    }

    #[test]
    fn it_parsers_a_to_integer_14() {
        assert_eq!(parse("AH"), 14);
    }

    #[test]
    fn it_parsers_any_other_to_integer_14() {
        assert_eq!(parse("XH"), 14);
        assert_eq!(parse("YH"), 14);
        assert_eq!(parse("ZH"), 14);
    }

    #[test]
    fn it_works_with_all_suits() {
        assert_eq!(parse("2H"), 2);
        assert_eq!(parse("2S"), 2);
        assert_eq!(parse("2D"), 2);
        assert_eq!(parse("2C"), 2);
    }
}
