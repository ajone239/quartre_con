use rand::seq::IteratorRandom;
use std::fmt::{Debug, Display};

use crate::{
    board::{board::BoardMove, piece::Piece},
    game::Play,
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

impl<E: Debug> Play<E> for Random {
    type MoveData = BoardMove;

    fn get_move(&self, board: &dyn crate::game::GameBoard<Self::MoveData, E>) -> Self::MoveData {
        board
            .list_moves()
            .into_iter()
            .choose(&mut rand::thread_rng())
            .map(|mut m| {
                m.add_color(self.color);
                m
            })
            .unwrap()
    }

    fn needs_to_see_board(&self) -> bool {
        false
    }

    fn should_announce_move(&self) -> bool {
        true
    }
}
