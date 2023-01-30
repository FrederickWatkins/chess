use std::{ops::Add, fmt::Display};

use crate::piece::*;
use array2d::Array2D;

mod board_layout;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Position {
    x: u8,
    y: u8,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Offset {
    x: isize,
    y: isize,
}

impl Add<Offset> for Position {
    type Output = Self;

    fn add(self, rhs: Offset) -> Self::Output {
        Self {
            x: (isize::from(self.x) + rhs.x).try_into().unwrap(),
            y: (isize::from(self.y) + rhs.y).try_into().unwrap(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
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

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Board {
    pieces: Array2D<Option<Piece>>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            pieces: board_layout::DEFAULT_BOARD.clone(),
        }
    }

    /// Moves piece from from_position to to_position, taking a piece if neccesary. Does not check if move is possible.
    pub fn move_piece(&mut self, from_position: Position, to_position: Position) {
        todo!()
    }

    /// Takes in the position of a piece, returns all possible positions it could move to. Returns none if piece does not exist.
    pub fn calculate_possible_moves(&self, position: Position) -> Option<Vec<Position>> {
        use Direction::*;
        let piece = match self.pieces[(
            position.x.try_into().unwrap(),
            position.y.try_into().unwrap(),
        )] {
            Some(piece) => piece,
            None => return None,
        };
        match piece.piece_type {
            PieceType::Pawn => todo!(),
            PieceType::Knight => todo!(),
            PieceType::Bishop => {
                self.check_directions(position, vec![NE, SE, SW, NW], piece.color);
            }
            PieceType::Rook => {
                self.check_directions(position, vec![N, E, S, W], piece.color);
            }
            PieceType::Queen => {
                self.check_directions(position, vec![N, NE, E, SE, S, SW, W, NW], piece.color);
            }
            PieceType::King => todo!(),
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

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod position_tests {
    use super::*;

    #[test]
    fn test_offset_positive_n() {
        assert_eq!(
            Position { x: 6, y: 6 },
            Position { x: 6, y: 5 } + Offset { x: 0, y: 1 }
        );
    }

    #[test]
    fn test_offset_positive_ne() {
        assert_eq!(
            Position { x: 6, y: 6 },
            Position { x: 5, y: 5 } + Offset { x: 1, y: 1 }
        );
    }

    #[test]
    fn test_offset_negative_s() {
        assert_eq!(
            Position { x: 6, y: 5 },
            Position { x: 6, y: 6 } + Offset { x: 0, y: -1 }
        );
    }

    #[test]
    fn test_offset_negative_sw() {
        assert_eq!(
            Position { x: 5, y: 5 },
            Position { x: 6, y: 6 } + Offset { x: -1, y: -1 }
        );
    }
}

#[cfg(test)]
mod board_tests {
    use super::*;

    mod calculate_possible_moves {
        use super::*;
    }

    mod check_directions {
        use super::*;

        #[test]
        fn cardinal_directions() {
            let board = Board::new();
            let mut result = board.check_directions(
                Position { x: 3, y: 4 },
                vec![Direction::N, Direction::E, Direction::S, Direction::W],
                Color::White,
            );
            result.sort();
            let mut expected_result = vec![
                Position { x: 0, y: 4 },
                Position { x: 1, y: 4 },
                Position { x: 2, y: 4 },
                Position { x: 4, y: 4 },
                Position { x: 5, y: 4 },
                Position { x: 6, y: 4 },
                Position { x: 7, y: 4 },
                Position { x: 3, y: 2 },
                Position { x: 3, y: 3 },
                Position { x: 3, y: 5 },
                Position { x: 3, y: 6 },
            ];
            expected_result.sort();
            assert_eq!(result, expected_result)
        }

        #[test]
        fn diagonal_directions() {
            let board = Board::new();
            let mut result = board.check_directions(
                Position { x: 4, y: 5 },
                vec![Direction::NE, Direction::SE, Direction::SW, Direction::NW],
                Color::Black,
            );
            result.sort();
            let mut expected_result = vec![
                Position { x: 0, y: 1 },
                Position { x: 1, y: 2 },
                Position { x: 2, y: 3 },
                Position { x: 3, y: 4 },
                Position { x: 7, y: 2 },
                Position { x: 6, y: 3 },
                Position { x: 5, y: 4 },
            ];
            expected_result.sort();
            assert_eq!(result, expected_result)
        }

        #[test]
        fn all_directions() {
            let board = Board::new();
            let mut result = board.check_directions(
                Position { x: 1, y: 3 },
                vec![Direction::N, Direction::NE, Direction::E, Direction::SE, Direction::S, Direction::SW, Direction::W, Direction::NW],
                Color::Black,
            );
            result.sort();
            let mut expected_result = vec![
                Position {x: 0, y: 3},
                Position {x: 2, y: 3},
                Position {x: 3, y: 3},
                Position {x: 4, y: 3},
                Position {x: 5, y: 3},
                Position {x: 6, y: 3},
                Position {x: 7, y: 3},
                Position {x: 1, y: 1},
                Position {x: 1, y: 2},
                Position {x: 1, y: 4},
                Position {x: 1, y: 5},
                Position {x: 0, y: 2},
                Position {x: 2, y: 4},
                Position {x: 3, y: 5},
                Position {x: 0, y: 4},
                Position {x: 2, y: 2},
                Position {x: 3, y: 1},

            ];
            expected_result.sort();
            assert_eq!(result, expected_result)
        }


    }

    mod check_direction {
        use super::*;

        #[test]
        fn no_move_n() {
            let board = Board::new();
            assert_eq!(
                board.check_direction(Position { x: 4, y: 0 }, Direction::N, Color::White),
                vec![]
            );
        }

        #[test]
        fn no_move_w() {
            let board = Board::new();
            assert_eq!(
                board.check_direction(Position { x: 5, y: 1 }, Direction::W, Color::White),
                vec![]
            );
        }

        #[test]
        fn edge_board_e() {
            let board = Board::new();
            assert_eq!(
                board.check_direction(Position { x: 6, y: 5 }, Direction::E, Color::White),
                vec![Position { x: 7, y: 5 }]
            );
        }

        #[test]
        fn edge_board_s() {
            let board = Board::new();
            assert_eq!(
                board.check_direction(Position { x: 3, y: 7 }, Direction::E, Color::White),
                vec![]
            );
        }

        #[test]
        fn take_piece_ne() {
            let board = Board::new();
            let mut result =
                board.check_direction(Position { x: 2, y: 2 }, Direction::NE, Color::White);
            result.sort();
            let mut expected_result = vec![
                Position { x: 3, y: 3 },
                Position { x: 4, y: 4 },
                Position { x: 5, y: 5 },
                Position { x: 6, y: 6 },
            ];
            expected_result.sort();
            assert_eq!(result, expected_result);
        }

        #[test]
        fn take_piece_sw() {
            let board = Board::new();
            let mut result =
                board.check_direction(Position { x: 4, y: 3 }, Direction::SW, Color::Black);
            result.sort();
            let mut expected_result = vec![Position { x: 3, y: 2 }, Position { x: 2, y: 1 }];
            expected_result.sort();
            assert_eq!(result, expected_result);
        }
    }
}
