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
    MinusInfinity,
    Lose,
    Draw,
    OnGoing(isize),
    Win,
    PlusInfinity,
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
            (Self::MinusInfinity, _) | (_, Self::PlusInfinity) => Ordering::Less,
            (Self::PlusInfinity, _) | (_, Self::MinusInfinity) => Ordering::Greater,
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
    fn evaluate(&self, use_threats: bool) -> GameEvaluation;
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
            let done = self.game_loops();
            if done {
                break;
            }
        }
    }

    fn game_loops(&mut self) -> bool {
        for player_num in &[1, 2] {
            let done = self.player_loop(*player_num);
            if done {
                return done;
            }
        }
        false
    }

    fn player_loop(&mut self, player_num: u32) -> bool {
        let p = match player_num {
            1 => &mut self.player1,
            2 => &mut self.player2,
            _ => unreachable!(),
        };
        if p.needs_to_see_board() {
            self.board.calculate_threats();
            println!("{}", self.board);
            self.board.clear_threats();
            println!("Please enter your move:");
        }

        let move_data = p.get_move(self.board.clone());

        if p.should_announce_move() {
            println!("Played {:?}", move_data);
        }

        let move_result = self.board.apply_move(&move_data);
        match move_result {
            Ok(t) => t,
            Err(e) => {
                println!(
                    "Move {:?} for player {} failed becuase: {:?}.",
                    move_data, p, e
                );
                println!("Exiting");
                println!("{}", self.board);
                return true;
            }
        };

        println!();
        println!();

        println!("Evaluating current board state");
        let eval = self.board.evaluate(false);

        match eval {
            GameEvaluation::Win => {
                print_boardered(&format!("{} beat {}!", self.player1, self.player2));
                println!("{}", self.board);
                true
            }
            GameEvaluation::Lose => {
                print_boardered(&format!("{} beat {}!", self.player2, self.player1));
                println!("{}", self.board);
                true
            }
            GameEvaluation::Draw => {
                print_boardered("It's a Draw!");
                println!("{}", self.board);
                true
            }
            GameEvaluation::OnGoing(val) => {
                println!("The game continues with the eval {}.", val);
                println!();
                false
            }
            _ => {
                println!("Invalid state");
                println!();
                true
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
