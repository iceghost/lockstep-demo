use std::{cmp::Ordering, fmt::Display};

#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Hand {
    Rock = 0,
    Paper = 1,
    Scissor = 2,
}

impl Hand {
    pub fn opposite(self) -> Self {
        [Hand::Rock, Hand::Paper, Hand::Scissor][(self as usize + 1) % 3]
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Hand::Rock => write!(f, "rock"),
            Hand::Paper => write!(f, "paper"),
            Hand::Scissor => write!(f, "scissor"),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            Ordering::Equal
        } else if *self == other.opposite() {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}
