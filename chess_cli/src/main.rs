use chess_lib::board;

fn main() {
    let mut b = board::Board::new();
    eprintln!(
        "{}",
        b.check_positions(board::Position::new(3, 3).unwrap())
            .unwrap_err()
    );
    eprintln!(
        "{}",
        b.move_piece(board::Position::new(0, 0).unwrap(), board::Position::new(0, 7).unwrap())
            .unwrap_err()
    );
    eprintln!("{}", board::Position::new(8, 1).unwrap_err())
}
