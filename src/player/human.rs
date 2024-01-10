use std::{
    fmt::{Debug, Display},
    io,
    str::FromStr,
};

use crate::game::{GameBoard, Play};

#[derive(Debug)]
pub struct Human {
    pub name: String,
}

impl Display for Human {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl<B, D, E: Debug> Play<B, D, E> for Human
where
    B: GameBoard<D, E>,
    D: FromStr,
{
    fn get_move(&self, board: &B) -> D {
        let stdin = io::stdin();
        let mut lines = stdin.lines();

        while let Some(Ok(line)) = lines.next() {
            match D::from_str(&line) {
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
