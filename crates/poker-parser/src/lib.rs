mod cards;

pub fn hi() {
    let card = cards::parse("2H").pop().unwrap();

    match card.suit {
        cards::card::Suit::Hearts => println!("H"),
        cards::card::Suit::Spades => println!("S"),
        _ => println!("OTHER")
    }
}
