use quatre_con::{
    board::{board::Board, piece::Piece},
    game::Game,
    player::human::Human,
};

fn main() {
    let mut g = Game {
        board: Box::new(Board::default()),
        players: vec![
            Box::new(Human { color: Piece::Red }),
            Box::new(Human {
                color: Piece::Yellow,
            }),
        ],
    };

    loop {
        for p in &g.players {
            let move_data = p.get_move();

            g.board.apply_move(move_data).unwrap();

            println!("{}", g.board);
        }
    }
}
