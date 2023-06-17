use lazy_static::lazy_static;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

lazy_static! {
    pub static ref SUITS: [Suit; 4] = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Suit {
    Diamonds,
    Hearts,
    Clubs,
    Spades,
}

impl FromStr for Suit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "clubs" => Ok(Suit::Clubs),
            "diamonds" => Ok(Suit::Diamonds),
            "hearts" => Ok(Suit::Hearts),
            "spades" => Ok(Suit::Spades),
            _ => Err(format!("Invalid rank: {}", s)),
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Suit::Clubs => "♣",
                Suit::Diamonds => "♦",
                Suit::Hearts => "♦",
                Suit::Spades => "♠",
            }
        )
    }
}
