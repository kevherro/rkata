#[derive(Debug, Clone)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Debug)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

#[derive(Debug, Eq, PartialEq)]
pub enum TablePosition {
    Early,
    Middle,
    Late,
}

#[derive(Debug)]
pub enum Action {
    Raise(StackSize),
    Call,
    Fold,
}

#[derive(Debug, Clone)]
pub enum StackSize {
    Short,
    Medium,
    Deep,
}

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
}
#[derive(Debug, Eq, PartialEq)]
pub enum HandStrength {
    EF0,
    EF1,
    EF2,
}

#[derive(Debug)]
pub struct Player {
    position: TablePosition,
    hand: Hand,
    stack_size: StackSize,
}

impl Player {
    pub fn new(position: TablePosition, hand: Hand, stack_size: StackSize) -> Player {
        Player {
            position,
            hand,
            stack_size,
        }
    }

    pub fn evaluate_hand(&self) -> HandStrength {
        // Placeholder algorithm.
        let rank1 = self.hand.cards[0].rank.to_owned();
        let rank2 = self.hand.cards[1].rank.to_owned();

        if rank1 == Rank::Eight && rank2 == Rank::Eight && self.position == TablePosition::Early {
            HandStrength::EF0
        } else {
            HandStrength::EF1
        }
    }

    pub fn act(&self, opponent_action: Action) -> Action {
        let hand_strength = self.evaluate_hand();

        // Placeholder algorithm.
        match self.position {
            TablePosition::Early => {
                if hand_strength == HandStrength::EF2 {
                    match opponent_action {
                        Action::Raise(_) => Action::Fold,
                        _ => Action::Raise(self.stack_size.to_owned()),
                    }
                } else {
                    Action::Fold
                }
            }
            TablePosition::Middle => {
                if hand_strength == HandStrength::EF0 {
                    match opponent_action {
                        Action::Raise(_) => Action::Fold,
                        _ => Action::Raise(self.stack_size.to_owned()),
                    }
                } else if hand_strength == HandStrength::EF1 {
                    Action::Call
                } else {
                    Action::Fold
                }
            }
            TablePosition::Late => {
                if hand_strength == HandStrength::EF0 {
                    match opponent_action {
                        Action::Raise(_) => Action::Fold,
                        _ => Action::Raise(self.stack_size.to_owned()),
                    }
                } else if hand_strength == HandStrength::EF1 {
                    Action::Call
                } else {
                    Action::Fold
                }
            }
        }
    }
}
