use core::panic;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
    marker::PhantomData,
};

use crate::game::{GameBoard, GameEvaluation, MoM};

#[derive(Debug)]
struct TreeNode<D: Debug + Clone> {
    eval: GameEvaluation,
    depth: usize,
    is_edge: bool,
    children: Vec<D>,
}

impl<D: Debug + Clone> TreeNode<D> {
    fn is_leaf(&self) -> bool {
        self.eval.is_terminal()
    }
}

impl<D: Clone + Debug> Display for TreeNode<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Eval: {:?}", self.eval)?;
        writeln!(f, "Depth: {}", self.depth)?;
        writeln!(f, "IsLeaf: {}", self.is_leaf())?;
        writeln!(f, "IsEdge: {}", self.is_edge)?;
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
            is_edge: true,
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

    pub fn walk_start(&mut self, mut board: B) {
        let node = self.tree_node_map.get(&board).unwrap();
        let start_depth = node.depth;

        self.walk_rec(&mut board, start_depth, 1);
    }

    fn walk_rec(&mut self, board: &mut B, start_depth: usize, depth: usize) {
        // Get the moves
        let moves = board.list_moves();
        let eval = board.evaluate();

        // Insert the board if needed
        let node = self.tree_node_map.entry(board.clone()).or_insert(TreeNode {
            eval,
            depth: start_depth + depth,
            is_edge: false,
            children: moves.clone(),
        });

        // If at depth then we are done
        if depth >= self.walk_depth || node.is_leaf() {
            node.is_edge = true;
            return;
        }

        node.is_edge = false;

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

    pub fn print_from_node(&self, board: &mut B) {
        let Some(node) = self.tree_node_map.get(&board) else {
            return;
        };

        // TODO(ajone239): add in indent prints
        // println!("{:indent$}Indented text!", "", indent=indent);
        println!("Board:");
        println!("{}", board);
        println!("Node:");
        println!("{}", node);

        // For each move
        for move_data in &node.children {
            // * apply move
            board
                .apply_move(&move_data)
                .expect("This should never fail as it is only valid moves");
            // * recurse
            self.print_from_node(board);
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

    pub fn get_best_move(&self, board: &mut B) -> D {
        let Some(node) = self.tree_node_map.get(&board) else {
            panic!("Attempted to use an unwalked board!");
        };
        let mut moves_w_evals = vec![];
        for m in &node.children {
            let f = |board: &mut _| {
                moves_w_evals.push((m, self.minimax(board)));
            };
            Tree::apply_recurse_remove(board, m, f);
        }

        match board.min_or_maxing() {
            MoM::Min => moves_w_evals
                .into_iter()
                .min_by(|x, y| x.1.cmp(&y.1))
                .map(|(m, _)| m)
                .unwrap()
                .clone(),
            MoM::Max => moves_w_evals
                .into_iter()
                .max_by(|x, y| x.1.cmp(&y.1))
                .map(|(m, _)| m)
                .unwrap()
                .clone(),
        }
    }

    fn minimax(&self, board: &mut B) -> GameEvaluation {
        let Some(node) = self.tree_node_map.get(&board) else {
            panic!("Attempted to use an unwalked board!");
        };

        if node.is_edge || node.is_leaf() {
            return node.eval;
        }

        let mut evals = vec![];
        for move_data in &node.children {
            let f = |board: &mut _| evals.push(self.minimax(board));
            Tree::apply_recurse_remove(board, move_data, f);
        }

        match board.min_or_maxing() {
            MoM::Min => evals.into_iter().min().unwrap(),
            MoM::Max => evals.into_iter().max().unwrap(),
        }
    }

    fn apply_recurse_remove<T, F>(board: &mut B, move_data: &D, f: F) -> T
    where
        F: FnOnce(&mut B) -> T,
    {
        // * apply move
        board
            .apply_move(&move_data)
            .expect("This should never fail as it is only valid moves");
        // * recurse
        let result = f(board);
        // * remove move
        board
            .remove_move(&move_data)
            .expect("This should never fail as it is only valid moves");

        result
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
