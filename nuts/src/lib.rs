use itertools::{iproduct, Itertools};
use rand::Rng;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Write};
use std::ops::Range;

const RANK_RANGE: Range<usize> = 2..15;
const SUIT_RANGE: Range<usize> = 0..4;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub rank: usize,
    pub suit: usize,
}

impl Card {
    fn new(rank: usize, suit: usize) -> Card {
        assert!(SUIT_RANGE.contains(&suit));
        assert!(RANK_RANGE.contains(&rank));
        Card { rank, suit }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        match self.rank {
            2 => output.write_str("2"),
            3 => output.write_str("3"),
            4 => output.write_str("4"),
            5 => output.write_str("5"),
            6 => output.write_str("6"),
            7 => output.write_str("7"),
            8 => output.write_str("8"),
            9 => output.write_str("9"),
            10 => output.write_str("10"),
            11 => output.write_str("J"),
            12 => output.write_str("Q"),
            13 => output.write_str("K"),
            14 => output.write_str("A"),
            _ => output.write_str(""),
        }
        .expect("TODO: panic message");

        output.write_str(" ").expect("TODO: panic message");

        match self.suit {
            0 => output.write_str("♠"),
            1 => output.write_str("♥"),
            2 => output.write_str("♦"),
            3 => output.write_str("♣"),
            _ => output.write_str(""),
        }
        .expect("TODO: panic message");

        write!(f, "{output}")
    }
}

#[allow(dead_code)]
pub struct Hand {
    pub cards: Vec<Card>,
}

#[allow(dead_code)]
pub fn find_nuts(community_cards: &[Card]) -> (HandRank, Hand) {
    let deck: Vec<Card> = iproduct!(SUIT_RANGE, RANK_RANGE)
        .map(|(suit, rank)| Card { rank, suit })
        .filter(|card| !community_cards.contains(card))
        .collect();

    let unseen = deck.iter().combinations(2);
    let mut best_hand: Option<Hand> = None;
    let mut best_rank: Option<HandRank> = None;

    for combo in unseen {
        let mut cards: Vec<Card> = community_cards.to_vec();
        cards.push(*combo[0]);
        cards.push(*combo[1]);
        cards.sort();

        for hand in cards.iter().combinations(5) {
            let hand = Hand::new(hand.into_iter().cloned().collect());
            let rank = hand.evaluate();

            if best_rank.is_none() || rank > best_rank.unwrap() {
                best_hand = Some(hand);
                best_rank = Some(rank);
            }
        }
    }

    (best_rank.unwrap(), best_hand.unwrap())
}

impl Hand {
    pub fn new(cards: Vec<Card>) -> Hand {
        assert_eq!(cards.len(), 5);
        Hand { cards }
    }

    pub fn random(size: usize) -> Hand {
        let mut ranks_rng = rand::thread_rng();
        let mut suits_rng = rand::thread_rng();
        let mut seen: HashSet<Card> = HashSet::new();
        while seen.len() < size {
            let random_rank: usize = ranks_rng.gen_range(RANK_RANGE);
            let random_suit: usize = suits_rng.gen_range(SUIT_RANGE);
            seen.insert(Card::new(random_rank, random_suit));
        }
        Hand {
            cards: seen.into_iter().collect(),
        }
    }

    fn evaluate(&self) -> HandRank {
        let mut ranks = [0; 13];
        let mut suits = [0; 4];
        let mut is_flush = false;
        let mut is_straight = false;

        for card in &self.cards {
            ranks[card.rank - 2] += 1;
            suits[card.suit] += 1;
        }

        let max_same_rank = *ranks.iter().max().unwrap();
        let max_same_suit = *suits.iter().max().unwrap();

        if max_same_suit == 5 {
            is_flush = true;
        }

        for window in ranks.windows(5) {
            if window.iter().all(|&x| x == 1) {
                is_straight = true;
                break;
            }
        }

        if ranks[0] == 1 && ranks[1..=4].iter().all(|&x| x == 1) && ranks[12] == 1 {
            is_straight = true;
        }

        match (is_straight, is_flush, max_same_rank) {
            (true, true, _) if self.cards.iter().max().unwrap().rank == 14 => HandRank::RoyalFlush,
            (true, true, _) => HandRank::StraightFlush,
            (_, _, 4) => HandRank::FourOfAKind,
            (_, _, 3) if ranks.iter().filter(|&&x| x == 2).count() > 0 => HandRank::FullHouse,
            (_, true, _) => HandRank::Flush,
            (true, _, _) => HandRank::Straight,
            (_, _, 3) => HandRank::ThreeOfAKind,
            (_, _, 2) if ranks.iter().filter(|&&x| x == 2).count() > 1 => HandRank::TwoPair,
            (_, _, 2) => HandRank::OnePair,
            _ => HandRank::HighCard,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

impl Display for HandRank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HandRank::HighCard => write!(f, "High Card"),
            HandRank::OnePair => write!(f, "One Pair"),
            HandRank::TwoPair => write!(f, "Two Pair"),
            HandRank::ThreeOfAKind => write!(f, "Three of a Kind"),
            HandRank::Straight => write!(f, "Straight"),
            HandRank::Flush => write!(f, "Flush"),
            HandRank::FullHouse => write!(f, "Full House"),
            HandRank::FourOfAKind => write!(f, "Four of a Kind"),
            HandRank::StraightFlush => write!(f, "Straight Flush"),
            HandRank::RoyalFlush => write!(f, "Royal Flush"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_royal_flush() {
        let community_cards = vec![
            Card { rank: 10, suit: 0 }, // 10 of clubs
            Card { rank: 11, suit: 0 }, // Jack of clubs
            Card { rank: 12, suit: 0 }, // Queen of clubs
            Card { rank: 13, suit: 0 }, // King of clubs
            Card { rank: 14, suit: 0 }, // Ace of clubs
        ];

        let (best_rank, best_hand) = find_nuts(&community_cards);

        assert_eq!(best_rank, HandRank::RoyalFlush);

        // Verify the cards in the best hand.
        assert_eq!(best_hand.cards.len(), 5);
        assert!(best_hand.cards.contains(&Card { rank: 10, suit: 0 }));
        assert!(best_hand.cards.contains(&Card { rank: 11, suit: 0 }));
        assert!(best_hand.cards.contains(&Card { rank: 12, suit: 0 }));
        assert!(best_hand.cards.contains(&Card { rank: 13, suit: 0 }));
        assert!(best_hand.cards.contains(&Card { rank: 14, suit: 0 }));
    }

    #[test]
    fn test_four_of_a_kind() {
        let community_cards = vec![
            Card { rank: 10, suit: 0 }, // 10 of clubs
            Card { rank: 10, suit: 1 }, // 10 of spades
            Card { rank: 7, suit: 2 },
            Card { rank: 2, suit: 0 },
            Card { rank: 3, suit: 2 },
        ];

        let (best_rank, best_hand) = find_nuts(&community_cards);

        assert_eq!(best_rank, HandRank::FourOfAKind);

        assert_eq!(best_hand.cards.len(), 5);
        assert!(best_hand.cards.contains(&Card { rank: 10, suit: 0 }));
        assert!(best_hand.cards.contains(&Card { rank: 10, suit: 1 }));
        assert!(best_hand.cards.contains(&Card { rank: 10, suit: 2 }));
        assert!(best_hand.cards.contains(&Card { rank: 10, suit: 3 }));
    }

    #[test]
    fn test_straight_flush() {
        let community_cards = vec![
            Card { rank: 2, suit: 0 }, // 10 of clubs
            Card { rank: 2, suit: 1 }, // 10 of spades
            Card { rank: 5, suit: 0 },
            Card { rank: 6, suit: 0 },
            Card { rank: 9, suit: 2 },
        ];

        let (best_rank, best_hand) = find_nuts(&community_cards);

        assert_eq!(best_rank, HandRank::StraightFlush);

        assert_eq!(best_hand.cards.len(), 5);
        assert!(best_hand.cards.contains(&Card { rank: 3, suit: 0 }));
        assert!(best_hand.cards.contains(&Card { rank: 4, suit: 0 }));
    }
}
