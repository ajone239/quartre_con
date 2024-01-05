use std::{fmt::Debug, io, usize};

use crate::{
    board::{board::BoardMove, piece::Piece},
    game::Play,
};

pub struct Human {
    pub color: Piece,
}

impl<E: Debug> Play<E> for Human {
    type MoveData = BoardMove;

    fn get_move(&self, board: &dyn crate::game::GameBoard<Self::MoveData, E>) -> Self::MoveData {
        let stdin = io::stdin();
        let mut lines = stdin.lines();

        while let Some(Ok(line)) = lines.next() {
            match line.parse::<usize>() {
                Ok(val) => {
                    let mut move_data: BoardMove = val.into();
                    move_data.add_color(self.color);
                    if !board.is_move_valid(&move_data) {
                        println!("Invalid Move for current game state.");
                        continue;
                    }
                    return move_data;
                }
                Err(err) => {
                    println!("Invalid text entry: {:?}", err);
                    continue;
                }
            }
        }
        panic!();
    }

    fn needs_to_see_board(&self) -> bool {
        true
    }

    fn should_announce_move(&self) -> bool {
        false
    }
}
