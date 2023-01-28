use crate::piece::*;
use array2d::Array2D;
use bevy::prelude::{Commands, Query};
use ux::u4;

mod board_layouts;

pub fn setup(mut commands: Commands) {
    commands.spawn(RookBundle::new(
        Color::WHITE,
        Position([u4::new(0), u4::new(0)]),
    ));
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
