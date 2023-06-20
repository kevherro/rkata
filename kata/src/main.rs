use clap::Parser;
use nuts::HandRank;
use std::collections::HashMap;
use std::io;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    skill: String,
}

fn main() {
    let args = Args::parse();

    if args.skill == "nuts" {
        nuts();
    }
}

fn nuts() {
    let hand_ranks: HashMap<usize, HandRank> = HashMap::from([
        (0, HandRank::HighCard),
        (1, HandRank::OnePair),
        (2, HandRank::TwoPair),
        (3, HandRank::ThreeOfAKind),
        (4, HandRank::Straight),
        (5, HandRank::Flush),
        (6, HandRank::FullHouse),
        (7, HandRank::FourOfAKind),
        (8, HandRank::StraightFlush),
        (9, HandRank::RoyalFlush),
    ]);

    loop {
        let hand = nuts::Hand::random(5);
        let cards = hand.cards.clone();
        println!();
        for card in cards {
            print!("{card}\t");
        }
        println!();
        println!();
        for (key, hand_rank) in &hand_ranks {
            println!("[{key}] {hand_rank}");
        }

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read line");

        let key: usize = match input.trim().parse() {
            Ok(key) => key,
            Err(_) => {
                println!("Invalid input. Please enter a valid number");
                return;
            }
        };

        match hand_ranks.get(&key) {
            Some(actual) => {
                let community_cards = hand.cards.clone();
                let expected = nuts::find_nuts(community_cards.as_slice());
                assert_eq!(actual.clone(), expected.0);
            }
            None => println!("Invalid input. Please enter a valid number"),
        };

        println!();
        println!();
    }
}
