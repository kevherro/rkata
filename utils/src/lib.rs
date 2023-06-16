use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

impl FromStr for Rank {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Rank::Ace),
            "2" => Ok(Rank::Two),
            "3" => Ok(Rank::Three),
            "4" => Ok(Rank::Four),
            "5" => Ok(Rank::Five),
            "6" => Ok(Rank::Six),
            "7" => Ok(Rank::Seven),
            "8" => Ok(Rank::Eight),
            "9" => Ok(Rank::Nine),
            "T" => Ok(Rank::Ten),
            "J" => Ok(Rank::Jack),
            "Q" => Ok(Rank::Queen),
            "K" => Ok(Rank::King),
            _ => Err(format!("Invalid rank: {}", s)),
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Rank::Ace => write!(f, "A"),
            Rank::Two => write!(f, "2"),
            Rank::Three => write!(f, "3"),
            Rank::Four => write!(f, "4"),
            Rank::Five => write!(f, "5"),
            Rank::Six => write!(f, "6"),
            Rank::Seven => write!(f, "7"),
            Rank::Eight => write!(f, "8"),
            Rank::Nine => write!(f, "9"),
            Rank::Ten => write!(f, "T"),
            Rank::Jack => write!(f, "J"),
            Rank::Queen => write!(f, "Q"),
            Rank::King => write!(f, "K"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Suit {
    Diamonds(String),
    Hearts(String),
    Clubs(String),
    Spades(String),
}

impl FromStr for Suit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "♣" => Ok(Suit::Clubs(s.to_string())),
            "♦" => Ok(Suit::Diamonds(s.to_string())),
            "♥" => Ok(Suit::Hearts(s.to_string())),
            "♠" => Ok(Suit::Spades(s.to_string())),
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
                Suit::Clubs(s) => s,
                Suit::Diamonds(s) => s,
                Suit::Hearts(s) => s,
                Suit::Spades(s) => s,
            }
        )
    }
}
