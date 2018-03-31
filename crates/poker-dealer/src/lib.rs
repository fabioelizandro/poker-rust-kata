#[derive(Debug, PartialEq)]
pub enum Suit {
    Hearts,
    Spades,
    Dimonds,
    Clubs
}

#[derive(Debug, PartialEq)]
pub struct Card {
    pub number: u8,
    pub suit: Suit
}

#[derive(Debug, PartialEq)]
pub struct PokerPlayer {
    pub id: String,
    pub cards: Vec<Card>
}

#[derive(Debug, PartialEq)]
pub struct PokerGame {
    pub players: Vec<PokerPlayer>
}

pub fn result(game: PokerGame) {
    println!("{:?}", game);
}
