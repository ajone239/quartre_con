use rand::seq::IteratorRandom;
use std::fmt::{Debug, Display};

use crate::{
    board::{
        board::{Board, BoardMove},
        piece::Piece,
    },
    game::{MovePiece, Play},
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

impl Play for Random {
    fn get_move(&mut self, board: Board) -> BoardMove {
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
