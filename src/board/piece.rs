use std::fmt::{Debug, Display};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Piece {
    Yellow,
    Red,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::Yellow => write!(f, "Y"),
            Piece::Red => write!(f, "R"),
        }
    }
}
