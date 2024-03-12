use std::fmt::{Debug, Display};

use thiserror::Error;

use super::{board_move::BoardMove, piece::Piece, square::Square};
use crate::game::{Evaluate, GameEvaluation, MoM, MovePiece};

const HEIGHT: usize = 6;
const WIDTH: usize = 7;

#[derive(Error, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Board {
    board: [[Square; WIDTH]; HEIGHT],
    turn_count: usize,
}

impl AsRef<Board> for Board {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Board {
    #[cfg(test)]
    fn new(board_str: &str) -> Self {
        let rows: Vec<&str> = board_str.split('\n').collect();

        let mut turn_count = 0;
        let mut board = [[Square::Empty; WIDTH]; HEIGHT];

        for (i, row) in rows.iter().rev().map(|l| l.trim()).enumerate() {
            let row_iter = row.chars().map(|c| match c {
                'R' => {
                    turn_count += 1;
                    Square::NonEmpty(Piece::Red)
                }
                'Y' => {
                    turn_count += 1;
                    Square::NonEmpty(Piece::Yellow)
                }
                '_' => Square::Empty,
                _ => unreachable!(),
            });

            for (j, c) in row_iter.enumerate() {
                board[i - 1][j] = c;
            }
        }

        Self { board, turn_count }
    }

    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
        !self.board.iter().any(|r| r.iter().any(|c| !c.is_empty()))
    }

    fn is_full(&self) -> bool {
        !self.board.iter().any(|r| r.iter().any(|c| c.is_empty()))
    }

    fn whos_to_play(&self) -> Piece {
        if self.turn_count & 1 == 0 {
            Piece::Yellow
        } else {
            Piece::Red
        }
    }

    fn eval_square(&self, i: usize, j: usize) -> SquareResult {
        let Square::NonEmpty(color) = self.board[i][j] else {
            return SquareResult::Empty;
        };

        let mut evals = match color {
            Piece::Yellow => [1; 4],
            Piece::Red => [-1; 4],
        };

        let mut blocked = [false; 4];

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
                let Some(square) = self.read_bounded(*i, *j) else {
                    continue;
                };

                let eval = match square {
                    Piece::Yellow => 1,
                    Piece::Red => -1,
                };

                blocked[d] |= square != color;

                evals[d] += eval;
            }
        }

        for (e, b) in evals.iter_mut().zip(blocked.iter()) {
            if *b {
                *e = 0;
            }
        }

        for e in evals {
            match e {
                4 => return SquareResult::Connect(Piece::Yellow),
                -4 => return SquareResult::Connect(Piece::Red),
                _ => continue,
            }
        }
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
    fn min_or_maxing(&self) -> MoM {
        match self.whos_to_play() {
            Piece::Red => MoM::Min,
            Piece::Yellow => MoM::Max,
        }
    }
    fn evaluate(&self) -> crate::game::GameEvaluation {
        let mut eval = 0;
        for (i, row) in self.board.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                let squares = self.eval_square(i, j);

                match squares {
                    SquareResult::Connect(Piece::Yellow) => return GameEvaluation::Win,
                    SquareResult::Connect(Piece::Red) => return GameEvaluation::Lose,
                    SquareResult::Disparate(val) => eval += val as isize,
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

        let color = self.whos_to_play();

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
        if let Some(color) = move_data.color {
            if self.whos_to_play() != color {
                return false;
            }
        }

        self.list_moves()
            .iter()
            .map(|m| m.column)
            .find(|i| move_data.column == *i)
            .is_some()
    }

    fn list_moves(&self) -> Vec<Self::MoveData> {
        let color = self.whos_to_play();

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

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::rstest;

    #[test]
    fn test_new() {
        let default_board = r"
        _______
        _______
        _______
        _______
        _______
        _______
        ";

        let test_board = Board::new(default_board);

        let expected_board = Board::default();

        assert_eq!(test_board, expected_board);
    }

    #[rstest]
    #[case(
        r"
        _______
        _______
        _______
        _______
        _______
        _______
        ",
        true
    )]
    #[case(
        r"
        _______
        _______
        _______
        _______
        _______
        Y______
        ",
        false
    )]
    fn test_is_empty(#[case] board_str: &str, #[case] expected: bool) {
        let test_board = Board::new(board_str);

        assert_eq!(expected, test_board.is_empty());
    }

    #[test]
    fn test_apply_move() {
        let mut board = Board::default();

        let expected_board = Board::new(
            r"
        _______
        _______
        _______
        _______
        _______
        Y______
        ",
        );

        assert!(board.is_empty());

        let move_data = BoardMove {
            column: 0,
            color: Some(Piece::Yellow),
        };

        board.apply_move(&move_data).unwrap();

        assert_eq!(board, expected_board);
    }

    #[rstest]
    #[case(
        r"
        _______
        _______
        _______
        _______
        _______
        Y______
        ",
        0,
        r"
        _______
        _______
        _______
        _______
        _______
        _______
        "
    )]
    #[case(
        r"
        _______
        _______
        _______
        _______
        R______
        Y______
        ",
        0,
        r"
        _______
        _______
        _______
        _______
        _______
        Y______
        "
    )]
    #[case(
        r"
        _______
        _______
        _______
        _______
        R______
        YY__R__
        ",
        4,
        r"
        _______
        _______
        _______
        _______
        R______
        YY_____
        "
    )]
    fn test_remove_move(#[case] init: &str, #[case] column: usize, #[case] expected: &str) {
        let mut init = Board::new(init);

        let expected = Board::new(expected);

        init.remove_move(&column.into()).unwrap();

        assert_eq!(init, expected);
    }

    #[test]
    fn test_remove_move_error() {
        let mut init = Board::default();

        assert_eq!(
            init.remove_move(&0usize.into()),
            Err(BoardError::InvalidMove(0))
        );

        assert_eq!(
            init.remove_move(&WIDTH.into()),
            Err(BoardError::OutOfRange(WIDTH))
        );
    }

    #[test]
    fn test_apply_move_out_of_range() {
        let mut board = Board::default();

        let move_data = BoardMove {
            column: WIDTH,
            color: Some(Piece::Yellow),
        };

        assert_eq!(
            board.apply_move(&move_data),
            Err(BoardError::OutOfRange(WIDTH))
        );
    }

    #[test]
    fn test_fill_empty() {
        let mut board = Board::default();

        assert!(board.is_empty());

        for _ in 0..(HEIGHT * WIDTH) {
            let move_data = *board.list_moves().first().unwrap();

            assert_eq!(board.whos_to_play(), move_data.color.unwrap());

            board.apply_move(&move_data).unwrap();
        }

        let move_data = BoardMove {
            column: 0,
            color: Some(Piece::Yellow),
        };

        assert_eq!(
            board.apply_move(&move_data),
            Err(BoardError::InvalidMove(0))
        );

        assert!(board.is_full());

        for _ in 0..HEIGHT {
            for j in 0..WIDTH {
                let move_data = BoardMove {
                    column: j,
                    color: None,
                };

                board.remove_move(&move_data).unwrap();
            }
        }
        assert!(board.is_empty());
    }
}
