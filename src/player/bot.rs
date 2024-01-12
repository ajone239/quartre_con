use std::fmt::{Debug, Display};

use crate::{
    board::{
        board::{Board, BoardError, BoardMove},
        piece::Piece,
    },
    game::{MovePiece, Play},
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
        let mut game_tree = Tree::new(board.clone(), 3);
        game_tree.walk_start(board);

        println!("{}", game_tree);

        Self { color, game_tree }
    }
}

impl Play for Bot {
    fn get_move(&mut self, board: Board) -> BoardMove {
        self.game_tree.walk_start(board.clone());

        self.game_tree.print_from_node(&mut board.clone());

        board.list_moves().into_iter().next().unwrap()
    }

    fn needs_to_see_board(&self) -> bool {
        false
    }

    fn should_announce_move(&self) -> bool {
        true
    }
}
