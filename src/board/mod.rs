use std::ops::Add;

use crate::piece::*;
use array2d::Array2D;
use ux::u4;

mod board_layout;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Position {
    x: u4,
    y: u4,
}

pub struct Offset {
    x: isize,
    y: isize,
}

impl Add<Offset> for Position {
    type Output = Self;

    fn add(self, rhs: Offset) -> Self::Output {
        let old_x: u8 = self.x.into();
        let old_y: u8 = self.y.into();
        let new_x: isize = isize::from(old_x) - rhs.x;
        let new_y: isize = isize::from(old_y) - rhs.y;
        Self {x: u4::new(new_x.try_into().unwrap()), y: u4::new(new_y.try_into().unwrap())}
    }
}

enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

pub struct Board {
    pieces: Array2D<Option<Piece>>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            pieces: board_layout::DEFAULT_BOARD.clone(),
        }
    }

    /// Takes in the position of a piece, returns all possible positions it could move to. Returns none if piece does not exist.
    fn calculate_possible_moves(&self, position: Position) -> Option<Vec<Position>> {
        let piece = match self.pieces[(
            position.x.try_into().unwrap(),
            position.y.try_into().unwrap(),
        )] {
            Some(piece) => piece,
            None => return None,
        };
        match piece.piece_type {
            PieceType::PAWN => todo!(),
            PieceType::KNIGHT => todo!(),
            PieceType::BISHOP => todo!(),
            PieceType::ROOK => todo!(),
            PieceType::QUEEN => todo!(),
            PieceType::KING => todo!(),
        }
    }

    fn check_directions(&self, position: Position, directions: Vec<Direction>) -> Vec<Position> {
        let mut out = vec![];

        for direction in directions {
            out.append(&mut self.check_direction(position, direction));
        };
        out
    }

    fn check_direction(&self, position: Position, direction: Direction) -> Vec<Position> {
        match direction {
            Direction::N => todo!(),
            Direction::NE => todo!(),
            Direction::E => todo!(),
            Direction::SE => todo!(),
            Direction::S => todo!(),
            Direction::SW => todo!(),
            Direction::W => todo!(),
            Direction::NW => todo!(),
        }
    }
}
