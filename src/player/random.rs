use rand::{distributions::Distribution, seq::IteratorRandom};
use std::fmt::Debug;

use crate::{
    board::{board::BoardMove, piece::Piece},
    game::Play,
};

pub struct Random {
    pub color: Piece,
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
