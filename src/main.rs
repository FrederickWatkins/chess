mod board;
mod piece;

fn main() {
    let b = board::Board::new();
    println!("{b}");
    eprintln!(
        "{}",
        b.calculate_possible_moves(board::Position::new(3, 3).unwrap())
            .unwrap_err()
    );
    eprintln!("{}", board::Position::new(8, 1).unwrap_err())
}
