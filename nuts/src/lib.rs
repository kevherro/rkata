use itertools::Itertools;

const RANKS: [&'static str; 13] = [
    "A", "K", "Q", "J", "10", "9", "8", "7", "6", "5", "4", "3", "2",
];

const SUITS: [&'static str; 4] = ["♠", "♣", "♦", "♥"];

pub struct Card {
    rank: &'static str,
    suit: &'static str,
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Deck {
        let cards: Vec<Card> = RANKS
            .iter()
            .cartesian_product(&SUITS)
            .map(|(&rank, &suit)| Card { rank, suit })
            .collect();

        Deck { cards }
    }
}
