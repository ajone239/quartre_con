use core::panic;
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
    marker::PhantomData,
};

use crate::game::{GameBoard, GameEvaluation, MoM};

#[derive(Debug)]
struct TreeNode<D: Debug + Clone> {
    depth: usize,
    is_edge: bool,
    children: Vec<D>,
}

impl<D: Clone + Debug> Display for TreeNode<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Depth: {}", self.depth)?;
        writeln!(f, "IsEdge: {}", self.is_edge)?;
        writeln!(f, "Children: {:?}", self.children)?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum Algorithm {
    MiniMax,
    AlphaBeta,
}

#[derive(Debug)]
pub struct Tree<B, D, E>
where
    D: Clone + Debug + Default,
    E: Debug,
    B: Hash + Eq + Clone + GameBoard<D, E>,
{
    init_position: B,
    tree_node_map: HashMap<B, TreeNode<D>>,
    walk_depth: usize,
    eval_call_count: RefCell<usize>,
    alg: Algorithm,
    ghost: PhantomData<E>,
}

impl<B, D, E> Tree<B, D, E>
where
    D: Clone + Debug + Default,
    E: Debug,
    B: Hash + Eq + Clone + GameBoard<D, E>,
{
    pub fn new(board: B, walk_depth: usize, alg: Algorithm) -> Self {
        let depth = 0;

        let mut tree_node_map = HashMap::new();

        let moves = board.list_moves();

        let root = TreeNode {
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
            eval_call_count: RefCell::new(0),
            alg,
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

        // Insert the board if needed
        let node = self.tree_node_map.entry(board.clone()).or_insert(TreeNode {
            depth: start_depth + depth,
            is_edge: false,
            children: moves.clone(),
        });

        // If at depth then we are done
        if depth >= self.walk_depth {
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
        *self.eval_call_count.borrow_mut() = 0;

        let (_, move_data) = match self.alg {
            Algorithm::MiniMax => self.minimax(board, None),
            Algorithm::AlphaBeta => {
                self.alpha_beta_minimax(board, None, GameEvaluation::Lose, GameEvaluation::Win)
            }
        };

        let val = *self.eval_call_count.borrow();

        println!("Evaluated {} times with {:?}", val, self.alg);

        move_data
    }

    fn minimax(&self, board: &mut B, move_to_get_here: Option<D>) -> (GameEvaluation, D) {
        let Some(node) = self.tree_node_map.get(&board) else {
            panic!("Attempted to use an unwalked board!");
        };

        // Return the nodes eval if it is terminal
        if node.is_edge {
            if move_to_get_here.is_none() {
                println!("{board}");
            }
            let move_data = move_to_get_here.expect("Trying to get move for a terminal position!");
            let eval = board.evaluate();
            *self.eval_call_count.borrow_mut() += 1;
            return (eval, move_data);
        }

        // Run minimax on all the children
        let mut evals = vec![];
        for m in &node.children {
            // In the recursion call minimax again and push the result to a local evals vector
            let f = |board: &mut _, move_data: &D| {
                evals.push(self.minimax(board, Some(move_data.clone())));
            };

            // Recurse Recurse
            Tree::apply_recurse_remove(board, m, f);
        }

        // Get the best eval for us
        // and get the move associated as we _might_ need it
        let (eval, move_data) = match board.min_or_maxing() {
            MoM::Max => evals.into_iter().max_by(|x, y| x.0.cmp(&y.0)).unwrap(),
            MoM::Min => evals.into_iter().min_by(|x, y| x.0.cmp(&y.0)).unwrap(),
        };

        // If `move_to_get_here` is None then we know that `get_best_move()` called it
        // so we know to use the move that we found not that we were told.
        //
        // Otherwise use the move that we were told as it is correct.
        let ret_move = match move_to_get_here {
            Some(m) => m,
            None => move_data,
        };

        (eval, ret_move)
    }

    fn apply_recurse_remove<T, F>(board: &mut B, move_data: &D, f: F) -> T
    where
        F: FnOnce(&mut B, &D) -> T,
    {
        // * apply move
        board
            .apply_move(&move_data)
            .expect("This should never fail as it is only valid moves");
        // * recurse
        let result = f(board, move_data);
        // * remove move
        board
            .remove_move(&move_data)
            .expect("This should never fail as it is only valid moves");

        result
    }

    fn alpha_beta_minimax(
        &self,
        board: &mut B,
        move_to_get_here: Option<D>,
        mut alpha: GameEvaluation,
        mut beta: GameEvaluation,
    ) -> (GameEvaluation, D) {
        // Grab the node
        let Some(node) = self.tree_node_map.get(&board) else {
            println!("{}", board);
            panic!("Attempted to use an unwalked board!");
        };

        // Return the nodes eval if it is terminal
        if node.is_edge {
            if move_to_get_here.is_none() {
                println!("{board}");
            }
            let move_data = move_to_get_here.expect("Trying to get move for a terminal position!");
            let eval = board.evaluate();
            *self.eval_call_count.borrow_mut() += 1;
            return (eval, move_data);
        }

        let (eval, move_data) = match board.min_or_maxing() {
            MoM::Max => {
                // Min val for max
                let mut eval: GameEvaluation = GameEvaluation::Lose;
                let mut move_data: D = Default::default();

                for m in node.children.iter() {
                    board.apply_move(m).unwrap();
                    let (temp_eval, temp_move_data) =
                        self.alpha_beta_minimax(board, Some(m.clone()), alpha, beta);
                    board.remove_move(m).unwrap();

                    if temp_eval > eval {
                        eval = temp_eval;
                        move_data = temp_move_data;
                    }

                    alpha = GameEvaluation::max(alpha, eval);

                    if beta <= alpha {
                        break;
                    }
                }
                (eval, move_data)
            }
            MoM::Min => {
                // Max val for min
                let mut eval: GameEvaluation = GameEvaluation::Win;
                let mut move_data: D = Default::default();

                for m in node.children.iter() {
                    board.apply_move(m).unwrap();
                    let (temp_eval, temp_move_data) =
                        self.alpha_beta_minimax(board, Some(m.clone()), alpha, beta);
                    board.remove_move(m).unwrap();

                    if temp_eval < eval {
                        eval = temp_eval;
                        move_data = temp_move_data;
                    }
                    beta = GameEvaluation::min(beta, eval);

                    if beta <= alpha {
                        break;
                    }
                }
                (eval, move_data)
            }
        };

        // If `move_to_get_here` is None then we know that `get_best_move()` called it
        // so we know to use the move that we found not that we were told.
        //
        // Otherwise use the move that we were told as it is correct.
        let ret_move = match move_to_get_here {
            Some(m) => m,
            None => move_data,
        };

        (eval, ret_move)
    }
}

impl<B, D, E> Display for Tree<B, D, E>
where
    D: Clone + Debug + Default,
    E: Debug,
    B: Hash + Eq + Clone + GameBoard<D, E>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = self.init_position.clone();

        self.fmt_rec(f, &mut board)
    }
}
