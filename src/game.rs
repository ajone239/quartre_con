use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
};

use crate::board::{board::Board, board_move::BoardMove};

pub trait MovePiece {
    type MoveData;
    type MoveError: Debug;

    fn apply_move(&mut self, move_data: &Self::MoveData) -> Result<(), Self::MoveError>;
    fn remove_move(&mut self, move_data: &Self::MoveData) -> Result<(), Self::MoveError>;
    fn is_move_valid(&self, move_data: &Self::MoveData) -> bool;
    fn list_moves(&self) -> Vec<Self::MoveData>;
}

pub enum MoM {
    Min,
    Max,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub enum GameEvaluation {
    Lose,
    Draw,
    OnGoing(isize),
    Win,
}

impl GameEvaluation {
    pub fn is_terminal(&self) -> bool {
        if let Self::OnGoing(_) = self {
            false
        } else {
            true
        }
    }
}

impl Ord for GameEvaluation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Lose, Self::Lose) | (Self::Win, Self::Win) | (Self::Draw, Self::Draw) => {
                Ordering::Equal
            }
            (Self::Lose, _) | (_, Self::Win) => Ordering::Less,
            (Self::Win, _) | (_, Self::Lose) => Ordering::Greater,
            (Self::Draw, Self::OnGoing(_)) => Ordering::Less,
            (Self::OnGoing(_), Self::Draw) => Ordering::Greater,
            (Self::OnGoing(s), Self::OnGoing(o)) => s.cmp(o),
        }
    }
}

pub trait Evaluate {
    fn min_or_maxing(&self) -> MoM;
    fn evaluate(&self) -> GameEvaluation;
}

pub trait GameBoard<D, E>: MovePiece<MoveData = D, MoveError = E> + Evaluate + Display {}
impl<D, E, T: MovePiece<MoveData = D, MoveError = E> + Evaluate + Display> GameBoard<D, E> for T {}

pub struct Game {
    pub board: Board,
    pub player1: Box<dyn Play>,
    pub player2: Box<dyn Play>,
}

pub trait Play: Display {
    fn get_move(&mut self, board: Board) -> BoardMove;
    fn needs_to_see_board(&self) -> bool;
    fn should_announce_move(&self) -> bool;
}

impl Game {
    pub fn game_loop(&mut self) {
        loop {
            for p in &mut [&mut self.player1, &mut self.player2] {
                if p.needs_to_see_board() {
                    println!("{}", self.board);
                    println!("Please enter your move:");
                }

                let move_data = p.get_move(self.board.clone());

                if p.should_announce_move() {
                    println!("Played {:?}", move_data);
                }

                self.board.apply_move(&move_data).unwrap();

                let eval = self.board.evaluate();

                match eval {
                    GameEvaluation::Win => {
                        print_boardered(&format!("{} beat {}!", self.player1, self.player2));
                        println!("{}", self.board);
                        return;
                    }
                    GameEvaluation::Lose => {
                        print_boardered(&format!("{} beat {}!", self.player2, self.player1));
                        println!("{}", self.board);
                        return;
                    }
                    GameEvaluation::Draw => {
                        print_boardered("It's a Draw!");
                        println!("{}", self.board);
                        return;
                    }
                    GameEvaluation::OnGoing(val) => {
                        println!("The game continues with the eval {}.", val);
                        println!();
                    }
                }
            }
        }
    }
}

fn print_boardered(s: &str) {
    let border = "*".repeat(20);
    println!();
    println!("{}", border);
    println!("{}", s);
    println!("{}", border);
    println!();
}
