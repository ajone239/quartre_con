use std::fmt::Display;

pub trait MovePiece: Display {
    type MoveData;
    type MoveError;

    fn apply_move(&mut self, move_data: Self::MoveData) -> Result<(), Self::MoveError>;
    fn remove_move(&mut self, move_data: Self::MoveData) -> Result<(), Self::MoveError>;
    fn list_moves(&self) -> Vec<Self::MoveData>;
}

pub trait GameBoard<D, E>: MovePiece<MoveData = D, MoveError = E> + Display {}

impl<D, E, T: MovePiece<MoveData = D, MoveError = E> + Display> GameBoard<D, E> for T {}

pub trait Play {
    type MoveData;

    fn get_move(&self) -> Self::MoveData;
}

pub struct Game<D, E> {
    pub board: Box<dyn MovePiece<MoveData = D, MoveError = E>>,
    pub players: Vec<Box<dyn Play<MoveData = D>>>,
}
