use std::{io, usize};

use crate::{
    board::{board::BoardMove, piece::Piece},
    game::Play,
};

pub struct Human {
    pub color: Piece,
}

impl Play for Human {
    type MoveData = BoardMove;

    fn get_move(&self) -> Self::MoveData {
        let stdin = io::stdin();
        let mut lines = stdin.lines();

        while let Some(Ok(line)) = lines.next() {
            match line.parse::<usize>() {
                Ok(val) => return Into::<BoardMove>::into((val, self.color)),
                Err(err) => {
                    println!("Invalid text entry: {:?}", err);
                    continue;
                }
            }
        }
        panic!();
    }
}
