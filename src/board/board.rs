use std::fmt::{Debug, Display};

use thiserror::Error;

use super::{piece::Piece, square::Square};

const WIDTH: usize = 7;
const HEIGHT: usize = 6;

#[derive(Error, Debug)]
pub enum BoardError {
    #[error("Move {0} can't be played on this board.")]
    InvalidMove(usize),
    #[error("Move {0} out of range for this board.")]
    OutOfRange(usize),
}

#[derive(Default, Debug)]
pub struct Board {
    board: [[Square; WIDTH]; HEIGHT],
}

pub trait MovePiece {
    type MoveData;
    type MoveError;

    fn apply_move(&mut self, move_data: Self::MoveData) -> Result<(), Self::MoveError>;
    fn remove_move(&mut self, move_data: Self::MoveData) -> Result<(), Self::MoveError>;
}

impl MovePiece for Board {
    type MoveData = (usize, Piece);
    type MoveError = BoardError;

    fn apply_move(&mut self, move_data: Self::MoveData) -> Result<(), Self::MoveError> {
        let column = move_data.0;
        let color = move_data.1;

        if column >= WIDTH {
            return Err(BoardError::OutOfRange(column));
        }

        let square: Option<&mut Square> = self
            .board
            .iter_mut()
            .rev()
            .map(|r| &mut r[column])
            .find(|c| c.is_empty());

        match square {
            None => Err(BoardError::InvalidMove(column)),
            Some(s) => {
                *s = Square::NonEmpty(color);
                Ok(())
            }
        }
    }

    fn remove_move(&mut self, move_data: Self::MoveData) -> Result<(), Self::MoveError> {
        let column = move_data.0;
        let _color = move_data.1;

        if column >= WIDTH {
            return Err(BoardError::OutOfRange(column));
        }

        let square: Option<&mut Square> = self
            .board
            .iter_mut()
            .map(|r| &mut r[column])
            .find(|c| !c.is_empty());

        match square {
            None => Err(BoardError::InvalidMove(column)),
            Some(s) => {
                *s = Square::Empty;
                Ok(())
            }
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.board.iter() {
            write!(f, "|")?;
            for cell in row {
                write!(f, " {}", cell)?;
            }
            writeln!(f, " |")?;
        }

        write!(f, "=")?;
        for i in 0..WIDTH {
            write!(f, " {}", i)?;
        }
        writeln!(f, " =")?;

        Ok(())
    }
}
