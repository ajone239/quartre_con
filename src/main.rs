use quatre_con::{
    board::{board::Board, piece::Piece},
    game::{Game, Play},
    player::{bot::Bot, human::Human, random::Random},
    tree::Algorithm,
};

use clap::Parser;

fn main() {
    let args = Args::parse();

    let board = Board::default();
    let player1 = player_from_args(
        Piece::Yellow,
        &board,
        &args.one_player,
        &args.one_player_alg,
        args.one_player_depth,
    );
    let player2 = player_from_args(
        Piece::Red,
        &board,
        &args.two_player,
        &args.two_player_alg,
        args.two_player_depth,
    );

    let mut g = Game {
        board,
        player1,
        player2,
    };

    g.game_loop();
}

/// Play Connect 4 with us!
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The type of player player1 will be
    #[arg(short, long, default_value_t = String::from("human"))]
    one_player: String,

    /// The alg for player1
    #[arg(long, default_value_t = String::from("alphabeta"))]
    one_player_alg: String,

    /// The depth for player1 3 is easy 8 is impossible
    #[arg(long, default_value_t = 5)]
    one_player_depth: usize,

    /// The type of player player2 will be
    #[arg(short, long, default_value_t = String::from("bot"))]
    two_player: String,

    /// The alg for player2
    #[arg(long, default_value_t = String::from("alphabeta"))]
    two_player_alg: String,

    /// The depth for player2 3 is easy 8 is impossible
    #[arg(long, default_value_t = 5)]
    two_player_depth: usize,
}

fn player_from_args(
    color: Piece,
    board: &Board,
    player: &str,
    alg: &str,
    depth: usize,
) -> Box<dyn Play> {
    match player {
        "human" => Box::new(Human {
            name: color.to_string(),
        }),
        "bot" => {
            let alg = match alg {
                "alphabeta" => Algorithm::AlphaBeta,
                "minimax" => Algorithm::MiniMax,
                _ => panic!("Invalid alg type"),
            };

            Box::new(Bot::new(color, board.clone(), depth, alg, false))
        }
        "random" => Box::new(Random { color }),
        _ => panic!("Invalid player type"),
    }
}
