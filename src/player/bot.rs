use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use crate::{
    board::{
        board::{Board, BoardError, BoardMove},
        piece::Piece,
    },
    game::{GameBoard, Play},
    tree::Tree,
};

#[derive(Debug)]
pub struct Bot {
    pub color: Piece,
    game_tree: Tree<Board, BoardMove, BoardError>,
}

impl Display for Bot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.color)
    }
}

impl Bot {
    pub fn new(color: Piece, board: Board) -> Self {
        let mut board = board.clone();
        let mut game_tree = Tree::new(board.clone(), 3);
        game_tree.walk_start(&mut board);

        println!("{}", game_tree);

        Self { color, game_tree }
    }
}

impl<B, D, E: Debug> Play<B, D, E> for Bot
where
    B: GameBoard<D, E>,
    D: FromStr,
{
    fn get_move(&self, board: &B) -> D {
        board.list_moves().into_iter().next().unwrap()
    }

    fn needs_to_see_board(&self) -> bool {
        false
    }

    fn should_announce_move(&self) -> bool {
        true
    }
}
