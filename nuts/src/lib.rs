use itertools::Itertools;
use lazy_static::lazy_static;
use rand::seq::SliceRandom;


use utils::{Rank, Suit};

const RANKS: [Rank; 13] = [
    Rank::Ace,
    Rank::King,
    Rank::Queen,
    Rank::Jack,
    Rank::Ten,
    Rank::Nine,
    Rank::Eight,
    Rank::Seven,
    Rank::Six,
    Rank::Five,
    Rank::Four,
    Rank::Three,
    Rank::Two,
];

lazy_static! {
    static ref SUITS: [Suit; 4] = [
        Suit::Clubs(String::from("♣")),
        Suit::Diamonds(String::from("♦")),
        Suit::Hearts(String::from("♥")),
        Suit::Spades(String::from("♠")),
    ];
}

#[derive(Eq, PartialEq, Clone)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let cards: Vec<Card> = RANKS
            .iter()
            .clone()
            .cartesian_product(SUITS.iter())
            .map(|(rank, suit)| Card {
                rank: rank.clone(),
                suit: suit.clone(),
            })
            .collect();

        Deck { cards }
    }

    fn shuffle(self) -> Vec<Card> {
        let mut rng = rand::thread_rng();
        let mut shuffled_deck = self.cards.clone();
        shuffled_deck.shuffle(&mut rng);
        shuffled_deck[..5].to_vec()
    }
}

enum HandRank {
    HighCard(u32),
    OnePair(u32),
    TwoPair(u32),
    ThreeOfAKind(u32),
    Straight(u32),
    Flush(u32),
    FullHouse(u32),
    FourOfAKind(u32),
    StraightFlush(u32),
    RoyalFlush,
}

struct Nuts {
    hand: [Card; 5],
    nuts: (Card, Card),
}

impl Nuts {
    fn new(&mut self, hand: [Card; 5]) {
        self.hand = hand;
        self.nuts = self.evaluate_hand();
    }

    fn evaluate_hand(&self) -> (Card, Card) {
        // TODO
        (self.hand[0].clone(), self.hand[1].clone())
    }

    fn is_nuts(&self, tuple: (Card, Card)) -> bool {
        match tuple {
            (card_1, card_2) if card_1 == self.nuts.0 => card_2 == self.nuts.1,
            (card_1, card_2) if card_1 == self.nuts.1 => card_2 == self.nuts.0,
            _ => false,
        }
    }
}
