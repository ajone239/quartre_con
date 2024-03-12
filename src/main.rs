use quatre_con::{
    board::{board::Board, piece::Piece},
    game::Game,
    player::{bot::Bot, human::Human},
    tree::Algorithm,
};

fn main() {
    let board = Board::default();
    let player1 = Human {
        name: "player1".to_string(),
    };
    let player2 = Bot::new(Piece::Red, board.clone(), Algorithm::AlphaBeta);

    let mut g = Game {
        board,
        player1: Box::new(player1),
        player2: Box::new(player2),
    };

    g.game_loop();
}
