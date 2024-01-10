use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

pub trait MovePiece {
    type MoveData;
    type MoveError: Debug;

    fn apply_move(&mut self, move_data: &Self::MoveData) -> Result<(), Self::MoveError>;
    fn remove_move(&mut self, move_data: &Self::MoveData) -> Result<(), Self::MoveError>;
    fn is_move_valid(&self, move_data: &Self::MoveData) -> bool;
    fn list_moves(&self) -> Vec<Self::MoveData>;
}

#[derive(Debug)]
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

pub trait GamePlayer<B, D, E>: Play<B, D, E> + Display
where
    B: GameBoard<D, E> + Display,
    D: FromStr,
{
}
impl<D: FromStr, E, B: GameBoard<D, E>, T: Play<B, D, E> + Display> GamePlayer<B, D, E> for T {}

pub struct Game<B, D, E>
where
    B: GameBoard<D, E>,
{
    pub board: B,
    pub player1: Box<dyn GamePlayer<B, D, E>>,
    pub player2: Box<dyn GamePlayer<B, D, E>>,
}

pub trait Play<B, D, E>
where
    B: GameBoard<D, E>,
    D: FromStr,
{
    fn get_move(&self, board: &B) -> D;
    fn needs_to_see_board(&self) -> bool;
    fn should_announce_move(&self) -> bool;
}

impl<B, D: Debug, E: Debug> Game<B, D, E>
where
    B: GameBoard<D, E> + AsRef<B>,
    D: FromStr,
{
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
