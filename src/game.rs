use std::fmt::{Debug, Display};

pub trait MovePiece {
    type MoveData;
    type MoveError: Debug;

    fn apply_move(&mut self, move_data: &Self::MoveData) -> Result<(), Self::MoveError>;
    fn remove_move(&mut self, move_data: &Self::MoveData) -> Result<(), Self::MoveError>;
    fn is_move_valid(&self, move_data: &Self::MoveData) -> bool;
    fn list_moves(&self) -> Vec<Self::MoveData>;
}

pub trait GameBoard<D, E>: MovePiece<MoveData = D, MoveError = E> + Display {}
impl<D, E, T: MovePiece<MoveData = D, MoveError = E> + Display> GameBoard<D, E> for T {}

pub struct Game<D, E> {
    pub board: Box<dyn GameBoard<D, E>>,
    pub players: Vec<Box<dyn Play<E, MoveData = D>>>,
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
            for p in &self.players {
                if p.needs_to_see_board() {
                    println!("{}", self.board);
                    println!("Please enter your move:");
                }

                let move_data = p.get_move(self.board.as_ref());

                if p.should_announce_move() {
                    println!("Played {:?}", move_data);
                }

                self.board.apply_move(&move_data).unwrap();
            }
        }
    }
}
