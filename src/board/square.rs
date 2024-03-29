use std::fmt::{Debug, Display};

use super::piece::Piece;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Square {
    #[default]
    Empty,
    NonEmpty(Piece),
}

impl Square {
    pub fn is_empty(&self) -> bool {
        *self == Self::Empty
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "_"),
            Self::NonEmpty(p) => write!(f, "{}", p),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::square::Piece;

    #[test]
    fn test_is_empty() {
        assert!(Square::Empty.is_empty());
        assert!(!Square::NonEmpty(Piece::Red).is_empty());
    }
}
