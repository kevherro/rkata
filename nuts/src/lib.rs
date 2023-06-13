pub struct Card {
    rank: &'static str,
    suit: &'static str,
}

pub struct Cards {
    deck: Vec<Card>,
}

impl Cards {
    fn new() -> Cards {
        // TODO: Add the rest of the deck.
        let deck: Vec<Card> = vec![
            Card {
                rank: "A",
                suit: "♠",
            },
            Card {
                rank: "A",
                suit: "♣",
            },
            Card {
                rank: "A",
                suit: "♦",
            },
            Card {
                rank: "A",
                suit: "♥",
            },
        ];

        Cards { deck }
    }
}
