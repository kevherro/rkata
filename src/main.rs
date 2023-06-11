use kata::{Action, Card, Hand, Player, Rank, StackSize, Suit, TablePosition};

fn main() {
    let position = TablePosition::Early;
    let hand = Hand {
        cards: vec![
            Card {
                rank: Rank::Eight,
                suit: Suit::Spades,
            },
            Card {
                rank: Rank::Eight,
                suit: Suit::Hearts,
            },
        ],
    };
    let stack_size = StackSize::Medium;
    let player = Player::new(position, hand, stack_size);

    let opponent_action = Action::Raise(StackSize::Deep);
    let action = player.act(opponent_action);

    println!("{:?}", action);
}
