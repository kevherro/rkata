use itertools::Itertools;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub(crate) rank: u8,
    pub(crate) suit: u8,
}

#[allow(dead_code)]
pub struct Hand {
    pub(crate) cards: Vec<Card>,
}

#[allow(dead_code)]
pub fn find_nuts(community_cards: &[Card]) -> (HandRank, Hand) {
    let mut deck: Vec<Card> = Vec::new();
    for suit in 0..4 {
        for rank in 2..15 {
            let card = Card { rank, suit };
            if !community_cards.contains(&card) {
                deck.push(card);
            }
        }
    }

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

    fn evaluate(&self) -> HandRank {
        let mut ranks = [0; 13];
        let mut suits = [0; 4];
        let mut is_flush = false;
        let mut is_straight = false;

        for card in &self.cards {
            ranks[card.rank as usize - 2] += 1;
            suits[card.suit as usize] += 1;
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
