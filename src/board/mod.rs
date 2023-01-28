use crate::piece::*;
use array2d::Array2D;
use bevy::prelude::{Commands, Query};
use ux::u4;

mod board_layouts;

pub fn setup(mut commands: Commands) {
    for (x, row_iter) in board_layouts::DEFAULT_BOARD.rows_iter().enumerate() {
        for (y, element) in row_iter.enumerate() {
            match element {
                Some((color, piece)) => spawn_piece(
                    &mut commands,
                    *piece,
                    *color,
                    Position([
                        u4::new(x.try_into().unwrap()),
                        u4::new(y.try_into().unwrap()),
                    ]),
                ),
                None => {}
            }
        }
    }
}

pub fn show_board(query: Query<(&Position, &Piece)>) {
    let mut board_arr = Array2D::filled_with(" ", 8, 8);
    for (position, piece) in query.iter() {
        board_arr[(
            position.0[0].try_into().unwrap(),
            position.0[1].try_into().unwrap(),
        )] = match *piece {
            Piece::PAWN => "P",
            Piece::KNIGHT => "N",
            Piece::BISHOP => "B",
            Piece::ROOK => "R",
            Piece::QUEEN => "Q",
            Piece::KING => "K",
        }
    }
    print_array2d(board_arr)
}

fn print_array2d(array: Array2D<&str>) {
    for row in array.rows_iter() {
        for element in row {
            print!("{}", element);
        }
        print!("\n")
    }
}
