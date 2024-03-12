use std::str::FromStr;

use super::piece::Piece;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Default)]
pub struct BoardMove {
    pub column: usize,
    pub color: Option<Piece>,
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

impl FromStr for BoardMove {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let column = s.parse::<usize>()?;
        Ok(column.into())
    }
}

#[cfg(test)]
mod board_move_tests {
    use super::*;

    #[test]
    fn test_from_usize_color() {
        let column = 0;
        let color = Piece::Red;

        let test = (column, color);

        let test_move: BoardMove = test.into();

        assert_eq!(
            test_move,
            BoardMove {
                column,
                color: Some(color)
            }
        )
    }

    #[test]
    fn test_from_usize() {
        let column = 0;

        let test_move: BoardMove = column.into();

        assert_eq!(
            test_move,
            BoardMove {
                column,
                color: None
            }
        )
    }

    #[test]
    fn test_from_add_color() {
        let column = 0;
        let color = Piece::Red;

        let mut test_move: BoardMove = column.into();

        assert_eq!(
            test_move,
            BoardMove {
                column,
                color: None
            }
        );

        test_move.add_color(color);

        assert_eq!(
            test_move,
            BoardMove {
                column,
                color: Some(color)
            }
        );
    }
}
