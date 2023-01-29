use crate::piece::*;
use array2d::Array2D;
use bevy::prelude::{Bundle, Commands, Component, Query, With};
use std::io::stdin;
use ux::u4;

mod board_layouts;

#[derive(Component)]
pub struct Board;

#[derive(Component)]
pub struct Turn(Color);

#[derive(Component)]
pub struct NextMove(Option<(Position, Position)>);

#[derive(Bundle)]
pub struct BoardBundle {
    turn: Turn,
    next_move: NextMove,
    _b: Board,
}

impl BoardBundle {
    fn new() -> Self {
        Self {
            turn: Turn(Color::WHITE),
            next_move: NextMove(None),
            _b: Board,
        }
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn(BoardBundle::new());
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

pub fn get_input(mut query: Query<(&mut NextMove, &Turn, With<Board>)>) {
    println!(
        "{}, please enter move",
        match (*query.single().1).0 {
            Color::WHITE => "White",
            Color::BLACK => "Black",
        }
    );
    let input = &mut String::new();
    stdin().read_line(input).unwrap();
    input.pop(); // Remove newline character
    let coords: Vec<&str> = input.split(" ").collect();
    assert_eq!(coords.len(), 4);
    let pos1 = [
        u4::new(coords[0].parse::<u8>().unwrap()),
        u4::new(coords[1].parse::<u8>().unwrap()),
    ];
    let pos2 = [
        u4::new(coords[2].parse::<u8>().unwrap()),
        u4::new(coords[3].parse::<u8>().unwrap()),
    ];
    query.single_mut().0 .0 = Some((Position(pos1), Position(pos2)));
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
