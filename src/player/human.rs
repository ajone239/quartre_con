use std::{
    fmt::{Debug, Display},
    io,
    str::FromStr,
};

use crate::{
    board::{board::Board, board_move::BoardMove},
    game::{MovePiece, Play},
};

#[derive(Debug)]
pub struct Human {
    pub name: String,
}

impl Display for Human {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Play for Human {
    fn get_move(&mut self, board: Board) -> BoardMove {
        let stdin = io::stdin();
        let mut lines = stdin.lines();

        while let Some(Ok(line)) = lines.next() {
            match BoardMove::from_str(&line) {
                Ok(val) => {
                    if !board.is_move_valid(&val) {
                        println!("Invalid Move for current game state.");
                        continue;
                    }
                    return val;
                }
                Err(_) => {
                    println!("Invalid text entry");
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
