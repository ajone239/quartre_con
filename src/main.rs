use quatre_con::board::{board::Board, board::MovePiece, piece::Piece};

fn main() {
    let mut b = Board::default();

    b.apply_move((0, Piece::Yellow)).unwrap();
    b.apply_move((0, Piece::Yellow)).unwrap();
    b.apply_move((0, Piece::Yellow)).unwrap();
    b.apply_move((0, Piece::Yellow)).unwrap();
    b.apply_move((1, Piece::Yellow)).unwrap();

    b.remove_move((0, Piece::Yellow)).unwrap();

    println!("{}", b);
}
