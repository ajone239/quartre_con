use std::fmt::{Debug, Display};

use thiserror::Error;

use super::{piece::Piece, square::Square};
use crate::game::MovePiece;

const WIDTH: usize = 7;
const HEIGHT: usize = 6;

#[derive(Error, Debug)]
pub enum BoardError {
    #[error("Move {0} can't be played on this board.")]
    InvalidMove(usize),
    #[error("Move {0} out of range for this board.")]
    OutOfRange(usize),
    #[error("Move failed to provide a color when it was needed.")]
    NoColor,
}

pub struct BoardMove {
    column: usize,
    color: Option<Piece>,
}

impl BoardMove {
    fn add_color(&mut self, color: Piece) {
        self.color = Some(color);
    }
}

impl From<(usize, Piece)> for BoardMove {
    fn from(value: (usize, Piece)) -> Self {
        BoardMove {
            column: value.0,
            color: Some(value.1),
        }
    }
}

impl From<usize> for BoardMove {
    fn from(value: usize) -> Self {
        BoardMove {
            column: value,
            color: None,
        }
    }
}

#[derive(Default, Debug)]
pub struct Board {
    board: [[Square; WIDTH]; HEIGHT],
}

impl MovePiece for Board {
    type MoveData = BoardMove;
    type MoveError = BoardError;

    fn apply_move(&mut self, move_data: Self::MoveData) -> Result<(), Self::MoveError> {
        let column = move_data.column;

        let Some(color) = move_data.color else {
            return Err(BoardError::NoColor);
        };

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
        let column = move_data.column;

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

    fn list_moves(&self) -> Vec<Self::MoveData> {
        self.board[HEIGHT - 1]
            .iter()
            .enumerate()
            .filter(|(_, s)| s.is_empty())
            .map(|(i, _)| i.into())
            .collect()
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
