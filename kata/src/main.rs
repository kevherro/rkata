use nuts::Hand;
use preflop::Context;
use rand::seq::SliceRandom;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::str::FromStr;

fn main() {
    let file_path: &str = "src/data/opening_hands_and_defending_raises";
    let mut contexts = match parse_file(file_path) {
        Ok(contexts) => contexts,
        Err(e) => {
            println!("Error parsing file: {:?}", e);
            return;
        }
    };

    let mut rng = rand::thread_rng();
    contexts.shuffle(&mut rng);

    for context in contexts.iter() {
        assess(context);
        println!();
    }
}

fn assess(context: &Context) {
    println!("{}", context);
    print!("action: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if context.assess(input.trim()) {
        println!("correct");
    } else {
        println!("incorrect. want: {}", context.get_expected_action());
    }
}

fn parse_file(file_path: &str) -> Result<Vec<Context>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let contexts = reader
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| Context::from_str(&line).ok())
        .collect();

    Ok(contexts)
}

fn nuts() {
    loop {
        let hand = Hand::new();
        if hand.is_nuts((0, 0)) {
            print!("correct")
        }
    }
}
