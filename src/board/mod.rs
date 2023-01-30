use std::ops::Add;

use crate::piece::*;
use array2d::Array2D;

mod board_layout;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Position {
    x: u8,
    y: u8,
}

pub struct Offset {
    x: isize,
    y: isize,
}

impl Add<Offset> for Position {
    type Output = Self;

    fn add(self, rhs: Offset) -> Self::Output {
        Self {
            x: (isize::from(self.x) - rhs.x).try_into().unwrap(),
            y: (isize::from(self.y) - rhs.y).try_into().unwrap(),
        }
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
        use Direction::*;
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
            PieceType::BISHOP => {
                self.check_directions(position, vec![NE, SE, SW, NW], piece.color);
            }
            PieceType::ROOK => {
                self.check_directions(position, vec![N, E, S, W], piece.color);
            }
            PieceType::QUEEN => {
                self.check_directions(position, vec![N, NE, E, SE, S, SW, W, NW], piece.color);
            }
            PieceType::KING => todo!(),
        }
        Some(vec![])
    }

    fn check_directions(
        &self,
        position: Position,
        directions: Vec<Direction>,
        color: Color,
    ) -> Vec<Position> {
        let mut out = vec![];

        for direction in directions {
            out.append(&mut self.check_direction(position, direction, color));
        }
        out
    }

    fn check_direction(
        &self,
        position: Position,
        direction: Direction,
        color: Color,
    ) -> Vec<Position> {
        let mut positions: Vec<Position> = vec![];
        let mut x_pos: isize = position.x.into();
        let mut y_pos: isize = position.y.into();
        let (x_offset, y_offset) = match direction {
            Direction::N => (0, 1),
            Direction::NE => (1, 1),
            Direction::E => (1, 0),
            Direction::SE => (1, -1),
            Direction::S => (0, -1),
            Direction::SW => (-1, -1),
            Direction::W => (-1, 0),
            Direction::NW => (-1, 1),
        };
        while 0 < x_pos && x_pos < 7 && 0 < y_pos && y_pos < 7 {
            x_pos += x_offset;
            y_pos += y_offset;
            let piece = if let Some(piece) =
                self.pieces[(x_pos.try_into().unwrap(), y_pos.try_into().unwrap())]
            {
                piece
            } else {
                positions.push(Position {
                    x: x_pos.try_into().unwrap(),
                    y: y_pos.try_into().unwrap(),
                });
                continue;
            };
            if piece.color == color {
                return positions;
            } else {
                positions.push(Position {
                    x: x_pos.try_into().unwrap(),
                    y: y_pos.try_into().unwrap(),
                });
                return positions;
            }
        }
        positions
    }
}
