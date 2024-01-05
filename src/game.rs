use std::fmt::{Debug, Display};

pub trait MovePiece {
    type MoveData;
    type MoveError: Debug;

    fn apply_move(&mut self, move_data: &Self::MoveData) -> Result<(), Self::MoveError>;
    fn remove_move(&mut self, move_data: &Self::MoveData) -> Result<(), Self::MoveError>;
    fn is_move_valid(&self, move_data: &Self::MoveData) -> bool;
    fn list_moves(&self) -> Vec<Self::MoveData>;
}

pub enum GameEvaluation {
    Win,
    Lose,
    Draw,
    OnGoing(f64),
}

pub trait Evaluate {
    fn evaluate(&self) -> GameEvaluation;
}

pub trait GameBoard<D, E>: MovePiece<MoveData = D, MoveError = E> + Evaluate + Display {}
impl<D, E, T: MovePiece<MoveData = D, MoveError = E> + Evaluate + Display> GameBoard<D, E> for T {}

pub trait GamePlayer<D, E>: Play<E, MoveData = D> + Display {}
impl<D, E, T: Play<E, MoveData = D> + Display> GamePlayer<D, E> for T {}

pub struct Game<D, E> {
    pub board: Box<dyn GameBoard<D, E>>,
    pub player1: Box<dyn GamePlayer<D, E>>,
    pub player2: Box<dyn GamePlayer<D, E>>,
}

pub trait Play<E> {
    type MoveData;

    fn get_move(&self, board: &dyn GameBoard<Self::MoveData, E>) -> Self::MoveData;
    fn needs_to_see_board(&self) -> bool;
    fn should_announce_move(&self) -> bool;
}

impl<D: Debug, E: Debug> Game<D, E> {
    pub fn game_loop(&mut self) {
        loop {
            for p in &[&self.player1, &self.player2] {
                if p.needs_to_see_board() {
                    println!("{}", self.board);
                    println!("Please enter your move:");
                }

                let move_data = p.get_move(self.board.as_ref());

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
