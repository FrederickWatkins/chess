use chess_lib::board;

fn main() {
    let b = board::Board::new();
    eprintln!(
        "{}",
        b.check_positions(board::Position::new(3, 3).unwrap())
            .unwrap_err()
    );
    eprintln!("{}", board::Position::new(8, 1).unwrap_err())
}
