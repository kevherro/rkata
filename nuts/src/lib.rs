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

pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Hand {
        let mut deck: Vec<Card> = RANKS
            .iter()
            .clone()
            .cartesian_product(SUITS.iter())
            .map(|(rank, suit)| Card {
                rank: rank.clone(),
                suit: suit.clone(),
            })
            .collect();

        deck.shuffle(&mut rand::thread_rng());

        Hand {
            cards: deck[..5].to_vec(),
        }
    }

    pub fn is_nuts(&self, cards: (usize, usize)) -> bool {
        let nuts = self.evaluate_hand();
        match cards {
            (card_1, card_2) if card_1 == nuts.0 => card_2 == nuts.1,
            (card_1, card_2) if card_1 == nuts.1 => card_2 == nuts.0,
            _ => false,
        }
    }

    fn evaluate_hand(&self) -> (usize, usize) {
        // TODO
        (0, 0)
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
