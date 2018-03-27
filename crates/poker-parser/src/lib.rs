mod cards;

pub fn hi() {
    let card = cards::card::parse("2H");

    match card.suit {
        cards::card::Suit::Hearts => println!("H"),
        cards::card::Suit::Spades => println!("S"),
        _ => println!("OTHER")
    }
}
