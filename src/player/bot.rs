use std::fmt::{Debug, Display};

use crate::{
    board::{
        board::{Board, BoardMove},
        piece::Piece,
    },
    game::Play,
    tree::Tree,
};

#[derive(Debug)]
pub struct Bot {
    pub color: Piece,
    game_tree: Tree<Board, BoardMove>,
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
        game_tree.walk_start(&mut board, 0);

        println!("{}", game_tree);

        Self { color, game_tree }
    }
}

impl<E: Debug> Play<E> for Bot {
    type MoveData = BoardMove;

    fn get_move(&self, board: &dyn crate::game::GameBoard<Self::MoveData, E>) -> Self::MoveData {
        board
            .list_moves()
            .into_iter()
            .map(|mut m| {
                m.add_color(self.color);
                m
            })
            .next()
            .unwrap()
    }

    fn needs_to_see_board(&self) -> bool {
        false
    }

    fn should_announce_move(&self) -> bool {
        true
    }
}
