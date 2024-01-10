use rand::seq::IteratorRandom;
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use crate::{
    board::piece::Piece,
    game::{GameBoard, Play},
};

#[derive(Debug)]
pub struct Random {
    pub color: Piece,
}

impl Display for Random {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.color)
    }
}

impl<B, D, E: Debug> Play<B, D, E> for Random
where
    B: GameBoard<D, E>,
    D: FromStr,
{
    fn get_move(&self, board: &B) -> D {
        board
            .list_moves()
            .into_iter()
            .choose(&mut rand::thread_rng())
            .unwrap()
    }

    fn needs_to_see_board(&self) -> bool {
        false
    }

    fn should_announce_move(&self) -> bool {
        true
    }
}
