use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
    marker::PhantomData,
};

use crate::game::{GameBoard, GameEvaluation};

#[derive(Debug)]
struct TreeNode<D: Debug + Clone> {
    eval: GameEvaluation,
    depth: usize,
    children: Vec<D>,
}

impl<D: Clone + Debug> Display for TreeNode<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Eval: {:?}", self.eval)?;
        writeln!(f, "Depth: {}", self.depth)?;
        writeln!(f, "Children: {:?}", self.children)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Tree<B, D, E>
where
    D: Clone + Debug,
    E: Debug,
    B: Hash + Eq + Clone + GameBoard<D, E>,
{
    init_position: B,
    tree_node_map: HashMap<B, TreeNode<D>>,
    walk_depth: usize,
    ghost: PhantomData<E>,
}

impl<B, D, E> Tree<B, D, E>
where
    D: Clone + Debug,
    E: Debug,
    B: Hash + Eq + Clone + GameBoard<D, E>,
{
    pub fn new(board: B, walk_depth: usize) -> Self {
        let depth = 0;

        let mut tree_node_map = HashMap::new();

        let moves = board.list_moves();
        let eval = board.evaluate();

        let root = TreeNode {
            eval,
            depth,
            children: moves,
        };

        let init_position = board.clone();

        tree_node_map.insert(board, root);

        Self {
            init_position,
            tree_node_map,
            walk_depth,
            ghost: PhantomData::default(),
        }
    }

    pub fn walk_start(&mut self, board: &mut B) {
        let start_depth = self.tree_node_map.get(&board).unwrap().depth;

        self.walk_rec(board, start_depth, 1);
    }

    fn walk_rec(&mut self, board: &mut B, start_depth: usize, depth: usize) {
        // Insert board and node
        let new_node_depth = start_depth + depth;
        let moves = board.list_moves();
        let eval = board.evaluate();

        let new_node = TreeNode {
            eval,
            depth: new_node_depth,
            children: moves.clone(),
        };

        self.tree_node_map.insert(board.clone(), new_node);

        // If at depth then we are done
        if depth >= self.walk_depth {
            return;
        }

        // For each move
        for move_data in moves {
            // * apply move
            board
                .apply_move(&move_data)
                .expect("This should never fail as it is only valid moves");
            // * recurse
            self.walk_rec(board, start_depth, depth + 1);
            // * remove move
            board
                .remove_move(&move_data)
                .expect("This should never fail as it is only valid moves");
        }
    }

    fn fmt_rec(&self, f: &mut std::fmt::Formatter<'_>, board: &mut B) -> std::fmt::Result {
        let Some(node) = self.tree_node_map.get(&board) else {
            return Ok(());
        };

        // TODO(ajone239): add in indent prints
        // println!("{:indent$}Indented text!", "", indent=indent);
        writeln!(f, "Board:")?;
        writeln!(f, "{}", board)?;
        writeln!(f, "Node:")?;
        writeln!(f, "{}", node)?;

        // For each move
        for move_data in &node.children {
            // * apply move
            board
                .apply_move(&move_data)
                .expect("This should never fail as it is only valid moves");
            // * recurse
            self.fmt_rec(f, board)?;
            // * remove move
            board
                .remove_move(&move_data)
                .expect("This should never fail as it is only valid moves");
        }

        Ok(())
    }
}

impl<B, D, E> Display for Tree<B, D, E>
where
    D: Clone + Debug,
    E: Debug,
    B: Hash + Eq + Clone + GameBoard<D, E>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = self.init_position.clone();

        self.fmt_rec(f, &mut board)
    }
}
