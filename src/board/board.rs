use std::fmt::{Debug, Display};

use thiserror::Error;

use super::{piece::Piece, square::Square};
use crate::game::{Evaluate, GameEvaluation, MovePiece};

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

enum SquareResult {
    Connect(Piece),
    Disparate(i64),
    Empty,
}

#[derive(Debug, Copy, Clone)]
pub struct BoardMove {
    column: usize,
    color: Option<Piece>,
}

impl BoardMove {
    pub fn add_color(&mut self, color: Piece) {
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

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Board {
    board: [[Square; WIDTH]; HEIGHT],
    turn_count: usize,
}

impl Board {
    #[cfg(test)]
    fn new(board_str: &str) -> Self {
        let rows: Vec<&str> = board_str.split('\n').collect();

        let mut board = [[Square::Empty; WIDTH]; HEIGHT];

        for (i, row) in rows.iter().enumerate() {
            let row: Vec<Square> = row
                .chars()
                .map(|c| match c {
                    'R' => Square::NonEmpty(Piece::Red),
                    'Y' => Square::NonEmpty(Piece::Yellow),
                    '_' => Square::Empty,
                    _ => unreachable!(),
                })
                .collect();

            board[i] = row.try_into().unwrap();
        }

        Self {
            board,
            turn_count: 0,
        }
    }

    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
        !self.board.iter().any(|r| r.iter().any(|c| !c.is_empty()))
    }

    fn is_full(&self) -> bool {
        !self.board.iter().any(|r| r.iter().any(|c| c.is_empty()))
    }

    fn eval_square(&self, i: usize, j: usize) -> SquareResult {
        let Square::NonEmpty(color) = self.board[i][j] else {
            return SquareResult::Empty;
        };

        let mut evals = match color {
            Piece::Yellow => [1; 4],
            Piece::Red => [-1; 4],
        };

        for k in 1..=3 {
            /*
             * *  *  *
             *  * * *
             *   ***
             *    ****
             */

            let directions = [
                // North West
                (i + k, j.overflowing_sub(k).0),
                // North
                (i + k, j),
                // North East
                (i + k, j + k),
                // East
                (i, j + k),
            ];

            for (d, (i, j)) in directions.iter().enumerate() {
                let square = match self.read_bounded(*i, *j) {
                    Some(Piece::Yellow) => 1,
                    Some(Piece::Red) => -1,
                    None => 0,
                };
                evals[d] += square;
            }
        }

        println!("{}:{}:", i, j);
        for e in evals {
            print!(" {}", e);
            match e {
                4 => return SquareResult::Connect(Piece::Yellow),
                -4 => return SquareResult::Connect(Piece::Red),
                _ => continue,
            }
        }
        println!();

        SquareResult::Disparate(evals.into_iter().sum())
    }
    fn read_bounded(&self, i: usize, j: usize) -> Option<Piece> {
        if i >= HEIGHT || j >= WIDTH {
            return None;
        }

        match self.board[i][j] {
            Square::Empty => None,
            Square::NonEmpty(color) => Some(color),
        }
    }
}

impl Evaluate for Board {
    fn evaluate(&self) -> crate::game::GameEvaluation {
        let mut eval = 0.0;
        for (i, row) in self.board.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                let sqres = self.eval_square(i, j);

                match sqres {
                    SquareResult::Connect(Piece::Yellow) => return GameEvaluation::Win,
                    SquareResult::Connect(Piece::Red) => return GameEvaluation::Lose,
                    SquareResult::Disparate(val) => eval += val as f64,
                    SquareResult::Empty => (),
                }
            }
        }

        if self.is_full() {
            return GameEvaluation::Draw;
        }

        GameEvaluation::OnGoing(eval)
    }
}

impl MovePiece for Board {
    type MoveData = BoardMove;
    type MoveError = BoardError;

    fn apply_move(&mut self, move_data: &Self::MoveData) -> Result<(), Self::MoveError> {
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
            .map(|r| &mut r[column])
            .find(|c| c.is_empty());

        match square {
            None => Err(BoardError::InvalidMove(column)),
            Some(s) => {
                self.turn_count += 1;
                *s = Square::NonEmpty(color);
                Ok(())
            }
        }
    }

    fn remove_move(&mut self, move_data: &Self::MoveData) -> Result<(), Self::MoveError> {
        let column = move_data.column;

        if column >= WIDTH {
            return Err(BoardError::OutOfRange(column));
        }

        let square: Option<&mut Square> = self
            .board
            .iter_mut()
            .rev()
            .map(|r| &mut r[column])
            .find(|c| !c.is_empty());

        match square {
            None => Err(BoardError::InvalidMove(column)),
            Some(s) => {
                self.turn_count -= 1;
                *s = Square::Empty;
                Ok(())
            }
        }
    }

    fn is_move_valid(&self, move_data: &Self::MoveData) -> bool {
        self.list_moves()
            .iter()
            .map(|m| m.column)
            .find(|i| move_data.column == *i)
            .is_some()
    }

    fn list_moves(&self) -> Vec<Self::MoveData> {
        let color = if self.turn_count & 1 == 0 {
            Piece::Yellow
        } else {
            Piece::Red
        };

        self.board[HEIGHT - 1]
            .iter()
            .enumerate()
            .filter(|(_, s)| s.is_empty())
            .map(|(i, _)| (i, color).into())
            .collect()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.board.iter().rev() {
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
        writeln!(f, " = {}", self.turn_count)?;

        Ok(())
    }
}
