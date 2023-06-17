use itertools::Itertools;
use rand::prelude::SliceRandom;
use std::collections::HashMap;
use utils::rank::Rank;
use utils::suit::Suit;

#[derive(Eq, PartialEq, Clone)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

pub struct Hand {
    #[allow(dead_code)]
    cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Hand {
        let mut deck: Vec<Card> = utils::rank::RANKS
            .iter()
            .clone()
            .cartesian_product(utils::suit::SUITS.iter())
            .map(|(rank, suit)| Card {
                rank: *rank,
                suit: *suit,
            })
            .collect();

        deck.shuffle(&mut rand::thread_rng());

        Hand {
            cards: deck[..5].to_vec(),
        }
    }
}

impl Default for Hand {
    fn default() -> Self {
        Hand::new()
    }
}

#[derive(Eq, PartialEq, PartialOrd)]
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

pub struct NutsEvaluator {}

impl NutsEvaluator {
    pub fn is_nuts(&self, cards: &[Card], hole_card1: Card, hole_card2: Card) -> bool {
        // Generate all possible combinations of community cards
        let community_combinations = cards.iter().combinations(5);

        // Check if the combination of hole cards and each community combination form a stronger hand
        for community_cards in community_combinations {
            let mut all_cards = vec![hole_card1.clone(), hole_card2.clone()];
            // all_cards.extend(community_cards.iter());
            all_cards.extend(community_cards.iter().map(|&card| card.clone()));

            let hand_rank = self.evaluate_hand(all_cards.as_slice());

            // Compare the hand rank of the current combination with the hole cards
            // If the rank is higher, it's not the nuts
            if hand_rank > self.evaluate_hand(&[hole_card1.clone(), hole_card2.clone()]) {
                return false;
            }
        }

        true
    }

    fn evaluate_hand(&self, hand: &[Card]) -> HandRank {
        let mut suits: HashMap<Suit, u32> = HashMap::new();
        let mut ranks: HashMap<Rank, u32> = HashMap::new();
        let mut rank_values: Vec<u32> = Vec::new();

        // Count the number of each suit and rank
        for card in hand {
            *suits.entry(card.clone().suit).or_insert(0) += 1;
            *ranks.entry(card.clone().rank).or_insert(0) += 1;
            rank_values.push(match card.clone().rank {
                Rank::Two => 2,
                Rank::Three => 3,
                Rank::Four => 4,
                Rank::Five => 5,
                Rank::Six => 6,
                Rank::Seven => 7,
                Rank::Eight => 8,
                Rank::Nine => 9,
                Rank::Ten => 10,
                Rank::Jack => 11,
                Rank::Queen => 12,
                Rank::King => 13,
                Rank::Ace => 14,
            })
        }

        // Check for hand categories in descending order of rank
        if let Some((_suit, _count)) = suits.iter().find(|(_, &count)| count >= 5) {
            // Flush check
            if let Some((rank, _count)) = ranks.iter().find(|(_, &count)| count >= 1) {
                // Straight flush check
                if let Some(straight_flush) = self.check_straight_flush(&rank_values, *rank) {
                    return straight_flush;
                }
            }
            // Flush
            return HandRank::Flush(*rank_values.iter().max().unwrap());
        }

        if let Some((_rank, _count)) = ranks.iter().find(|(_, &count)| count == 4) {
            // Four of a kind
            return HandRank::FourOfAKind(rank_values.iter().sum());
        }

        if let Some((_rank, _count)) = ranks.iter().find(|(_, &count)| count == 3) {
            // Three of a kind
            if ranks.iter().any(|(_, &count)| count >= 2) {
                // Full house
                return HandRank::FullHouse(rank_values.iter().sum());
            }
            return HandRank::ThreeOfAKind(rank_values.iter().sum());
        }

        if let Some(rank) = self.check_straight(&rank_values) {
            // Straight
            return HandRank::Straight(rank);
        }

        if let Some((rank, _count)) = ranks.iter().find(|(_, &count)| count == 2) {
            // Two pair
            if let Some(_second_rank) = ranks
                .iter()
                .find(|(other_rank, &count)| count == 2 && *rank != **other_rank)
            {
                return HandRank::TwoPair(rank_values.iter().sum());
            }
            // One pair
            return HandRank::OnePair(rank_values.iter().sum());
        }

        // High card
        HandRank::HighCard(*rank_values.iter().max().unwrap())
    }

    fn check_straight_flush(&self, ranks: &[u32], start_rank: Rank) -> Option<HandRank> {
        if let Some(rank) = self.check_straight(ranks) {
            if rank >= start_rank as u32 {
                if rank == 14 {
                    return Some(HandRank::RoyalFlush);
                }
                return Some(HandRank::StraightFlush(rank));
            }
        }
        None
    }

    fn check_straight(&self, ranks: &[u32]) -> Option<u32> {
        let mut sorted_ranks = ranks.to_vec();
        sorted_ranks.sort();
        let mut consecutive_ranks = 0;
        let mut previous_rank = 0;

        for &rank in &sorted_ranks {
            if rank == previous_rank + 1 {
                consecutive_ranks += 1;
            } else if rank != previous_rank {
                consecutive_ranks = 1;
            }
            previous_rank = rank;
            if consecutive_ranks == 5 {
                return Some(rank);
            }
        }
        None
    }
}
