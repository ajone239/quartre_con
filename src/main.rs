use quatre_con::{
    board::{board::Board, piece::Piece},
    game::Game,
    player::{bot::Bot, human::Human},
};

fn main() {
    let board = Board::default();
    let player1 = Human {
        color: Piece::Yellow,
    };
    let player2 = Bot::new(Piece::Red, board.clone());

    let mut g = Game {
        board: Box::new(board),
        player1: Box::new(player1),
        player2: Box::new(player2),
    };

    g.game_loop();
}
