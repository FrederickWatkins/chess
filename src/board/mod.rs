use crate::piece::*;
use array2d::Array2D;
use bevy::prelude::{Commands, Entity, Query, Component};
use std::io::stdin;
use ux::u4;

mod board_layouts;

#[derive(Component)]
struct Board;



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

pub fn take_turn(mut commands: Commands, mut query: Query<(Entity, &mut Position, &Piece, &Color)>) {
    let input = &mut String::new();
    stdin().read_line(input).unwrap();
    input.pop(); // Remove newline character
    let coords: Vec<&str> = input.split(" ").collect();
    assert_eq!(coords.len(), 4);
    let pos1 = (
        u4::new(coords[0].parse::<u8>().unwrap()),
        u4::new(coords[1].parse::<u8>().unwrap()),
    );
    let pos2 = (
        u4::new(coords[2].parse::<u8>().unwrap()),
        u4::new(coords[3].parse::<u8>().unwrap()),
    );
    let mut found = false;
    let mut possible = false;
    for (_entity, position, piece, color) in query.iter_mut() {
        if position.0 == [pos1.0, pos1.1] {
            found = true;
            possible = check_move(&position, piece);
            break;
        }
    }
    if found && possible {
        for (entity, position, _piece, color) in query.iter_mut() {
            if position.0 == [pos2.0, pos2.1] {
                commands.entity(entity).despawn();
            }
        }
    }
    else {
        panic!()
    }
}

/// Returns true if possible, false if impossible
fn check_move(position: &Position, piece: &Piece) -> bool {
    return true; // TODO implement logic for checking possible moves
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
    for (i, row) in array.rows_iter().enumerate() {
        print!("{}  ", i + 1);
        for element in row {
            print!("{element}  ");
        }
        print!("\n\n")
    }
    println!("   A  B  C  D  E  F  G  H")
}
