use quatre_con::{
    board::{board::Board, piece::Piece},
    game::Game,
    player::bot::Bot,
    tree::Algorithm,
};

fn main() {
    let board = Board::default();
    let player1 = Bot::new(Piece::Yellow, board.clone(), 5, Algorithm::AlphaBeta, false);
    let player2 = Bot::new(Piece::Red, board.clone(), 5, Algorithm::AlphaBeta, false);

    let mut g = Game {
        board,
        player1: Box::new(player1),
        player2: Box::new(player2),
    };

    g.game_loop();
}
