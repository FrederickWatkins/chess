mod board;
mod piece;

fn main() {
    let b = board::Board::new();
    println!("{b}");
    eprintln!(
        "{}",
        b.calculate_possible_moves(board::Position::new(3, 3))
            .unwrap_err()
    );
}
