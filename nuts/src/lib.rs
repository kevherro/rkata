mod nuts;

#[cfg(test)]
mod tests {
    use crate::nuts::{find_nuts, Card, HandRank};

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
