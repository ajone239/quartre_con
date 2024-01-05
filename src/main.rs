use quatre_con::{
    board::{board::Board, piece::Piece},
    game::Game,
    player::{human::Human, random::Random},
};

fn main() {
    let mut g = Game {
        board: Box::new(Board::default()),
        players: vec![
            Box::new(Human { color: Piece::Red }),
            Box::new(Random {
                color: Piece::Yellow,
            }),
        ],
    };

    g.game_loop();
}
