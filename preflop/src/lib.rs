use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
enum TablePosition {
    Early,
    Middle,
    Late,
}

impl FromStr for TablePosition {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "early" => Ok(TablePosition::Early),
            "middle" => Ok(TablePosition::Middle),
            "late" => Ok(TablePosition::Late),
            _ => Err(format!("Invalid table position: {}", s)),
        }
    }
}

impl Display for TablePosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TablePosition::Early => write!(f, "early"),
            TablePosition::Middle => write!(f, "middle"),
            TablePosition::Late => write!(f, "late"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Action {
    Raise(StackSize),
    Call,
    Fold,
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "r" => Ok(Action::Raise(StackSize::Short)),
            "f" => Ok(Action::Fold),
            "c" => Ok(Action::Call),
            _ => Err(format!("Invalid action: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StackSize {
    Short,
    Medium,
    Deep,
    Any,
}

impl FromStr for StackSize {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "s" => Ok(StackSize::Short),
            "m" => Ok(StackSize::Medium),
            "d" => Ok(StackSize::Deep),
            "a" => Ok(StackSize::Any),
            _ => Err(format!("Invalid stack size: {}", s)),
        }
    }
}

impl Display for StackSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StackSize::Short => write!(f, "short"),
            StackSize::Medium => write!(f, "medium"),
            StackSize::Deep => write!(f, "deep"),
            StackSize::Any => write!(f, "any"),
        }
    }
}

#[derive(Debug)]
enum SuitCombination {
    Suited,
    Offsuit,
}

impl FromStr for SuitCombination {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "s" => Ok(SuitCombination::Suited),
            "o" => Ok(SuitCombination::Offsuit),
            _ => Err(format!("Invalid suit combination: {}", s)),
        }
    }
}

impl Display for SuitCombination {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SuitCombination::Offsuit => write!(f, "offsuit"),
            SuitCombination::Suited => write!(f, "suited"),
        }
    }
}

#[derive(Debug)]
pub struct Context {
    table_position: TablePosition,
    card_1: utils::Rank,
    card_2: utils::Rank,
    suit_combination: SuitCombination,
    action: Action,
    stack_size: StackSize,
}

impl Context {
    pub fn assess(&self, action: &str) -> bool {
        let action: Action = match action.trim().parse() {
            Ok(action) => action,
            Err(_) => return false,
        };

        self.action == action
    }
    pub fn get_expected_action(&self) -> &str {
        match self.action {
            Action::Raise(_) => "raise",
            Action::Call => "call",
            Action::Fold => "fold",
        }
    }
}

impl FromStr for Context {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 6 {
            return Err("Invalid string format".to_string());
        }

        let table_position: TablePosition = match parts[0].trim().parse() {
            Ok(position) => position,
            Err(e) => return Err(e),
        };

        let card_1: utils::Rank = match parts[1].trim().parse() {
            Ok(rank) => rank,
            Err(e) => return Err(e),
        };

        let card_2: utils::Rank = match parts[2].trim().parse() {
            Ok(rank) => rank,
            Err(e) => return Err(e),
        };

        let suit_combination: SuitCombination = match parts[3].trim().parse() {
            Ok(suit) => suit,
            Err(e) => return Err(e),
        };

        let action: Action = match parts[4].trim().parse() {
            Ok(action) => action,
            Err(e) => return Err(e),
        };

        let stack_size: StackSize = match parts[5].trim().parse() {
            Ok(size) => size,
            Err(e) => return Err(e),
        };

        Ok(Context {
            table_position,
            card_1,
            card_2,
            suit_combination,
            action,
            stack_size,
        })
    }
}

impl Display for Context {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} pos, {}{} {}, {} stack",
            self.table_position, self.card_1, self.card_2, self.suit_combination, self.stack_size
        )
    }
}
