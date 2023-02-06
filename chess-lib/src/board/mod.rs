use crate::piece::*;
use array2d::Array2D;
use log::{debug, info, trace, warn};
use std::{
    fmt::Display,
    ops::{Add, Index, IndexMut},
};
use thiserror::Error;

mod board_layout;

#[derive(Error, Debug)]
#[error("No piece found at {position}.")]
pub struct PieceNotFound {
    position: Position,
}

#[derive(Error, Debug)]
#[error("Attempted to create position at {x}, {y}. Position x and y must both be less than 8")]
pub struct PositionOutOfBounds {
    x: isize,
    y: isize,
}

#[derive(Error, Debug)]
#[error("Attempted to create offset of {x}, {y}. Position x and y must both be less than 8 and more than -8")]
pub struct OffsetOutOfBounds {
    x: i8,
    y: i8,
}

/// Position on chess board
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Position {
    x: u8,
    y: u8,
}

impl Position {
    pub fn new(x: u8, y: u8) -> Result<Self, PositionOutOfBounds> {
        if x < 8 && y < 8 {
            Ok(Self { x, y })
        } else {
            Err(PositionOutOfBounds {
                x: x.into(),
                y: y.into(),
            })
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// Offset to position
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Offset {
    x: i8,
    y: i8,
}

impl Offset {
    pub fn new(x: i8, y: i8) -> Result<Self, OffsetOutOfBounds> {
        if -8 < x && x < 8 && -8 < y && y < 8 {
            Ok(Self { x, y })
        } else {
            Err(OffsetOutOfBounds { x, y })
        }
    }
}

impl Display for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add<Offset> for Position {
    type Output = Result<Self, PositionOutOfBounds>;

    fn add(self, rhs: Offset) -> Self::Output {
        let (new_x, new_y) = unsafe {
            (
                i8::try_from(self.x).unwrap_unchecked() + rhs.x,
                i8::try_from(self.y).unwrap_unchecked() + rhs.y,
            )
        }; // This is okay since x and y must always be less than 8
        Self::new(
            match new_x.try_into() {
                Ok(x) => x,
                Err(_) => {
                    return Err(PositionOutOfBounds {
                        x: new_x.into(),
                        y: new_y.into(),
                    });
                }
            },
            match new_y.try_into() {
                Ok(y) => y,
                Err(_) => {
                    return Err(PositionOutOfBounds {
                        x: new_x.into(),
                        y: new_y.into(),
                    });
                }
            },
        )
    }
}

/// Directions a piece can move
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

/// Chess board. It is the responsibility of the caller to ensure moves on the board are possible.
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

    /// Moves piece from from_position to to_position, taking a piece at the destination if neccesary. Does not check if move is possible.
    pub fn move_piece(
        &mut self,
        from_position: Position,
        to_position: Position,
    ) -> Result<(), PieceNotFound> {
        info!("Moving piece from {from_position} to {to_position}");
        self[to_position] = None;
        let mut piece = if let Some(piece) = self[from_position] {
            piece
        } else {
            return Err(PieceNotFound {
                position: from_position,
            });
        };
        piece.moved = true;
        self[from_position] = Some(piece);
        self[to_position] = self[from_position];
        self[from_position] = None;
        Ok(())
    }

    /// Takes in the position of a piece, returns all possible positions it could move to. Returns none if piece does not exist.
    pub fn calculate_possible_moves(
        &self,
        position: Position,
    ) -> Result<Vec<Position>, PieceNotFound> {
        use Direction::*;
        info!("Calculating possible moves for piece at {position}");
        let piece = if let Some(piece) = self[position] {
            debug!("Piece type is {:?}", piece.piece_type);
            piece
        } else {
            warn!("No piece found at {position}");
            return Err(PieceNotFound { position });
        };
        Ok(match piece.piece_type {
            PieceType::Pawn => self.check_pawn(position, piece.color, piece.moved),
            PieceType::Knight => self.check_knight(position, piece.color),
            PieceType::Bishop => self.check_directions(position, vec![NE, SE, SW, NW], piece.color),
            PieceType::Rook => self.check_directions(position, vec![N, E, S, W], piece.color),
            PieceType::Queen => {
                self.check_directions(position, vec![N, NE, E, SE, S, SW, W, NW], piece.color)
            }
            PieceType::King => self.check_king(position, piece.color),
        })
    }

    /// Checks directions and returns vector of possible positions.
    fn check_directions(
        &self,
        position: Position,
        directions: Vec<Direction>,
        color: Color,
    ) -> Vec<Position> {
        debug!("Checking directions {directions:?} for piece at {position} with color {color:?}");
        let mut out = vec![];
        for direction in directions {
            out.append(&mut self.check_direction(position, direction, color));
        }
        out
    }

    /// Checks direction and returns vector of possible positions.
    fn check_direction(
        &self,
        mut position: Position,
        direction: Direction,
        color: Color,
    ) -> Vec<Position> {
        debug!("Checking direction {direction:?} for piece at {position} with color {color:?}");
        let mut positions: Vec<Position> = vec![];
        let offset = match direction {
            Direction::N => Offset { x: 0, y: 1 },
            Direction::NE => Offset { x: 1, y: 1 },
            Direction::E => Offset { x: 1, y: 0 },
            Direction::SE => Offset { x: 1, y: -1 },
            Direction::S => Offset { x: 0, y: -1 },
            Direction::SW => Offset { x: -1, y: -1 },
            Direction::W => Offset { x: -1, y: 0 },
            Direction::NW => Offset { x: -1, y: 1 },
        };
        loop {
            position = if let Ok(position) = position + offset {
                position
            } else {
                break;
            };
            let piece = if let Some(piece) = self[position] {
                piece
            } else {
                positions.push(position);
                continue;
            };
            if piece.color == color {
                trace!("Reached piece of own color at {position}");
                return positions;
            } else {
                trace!("Reached piece of opposite color at {position}");
                positions.push(position);
                return positions;
            }
        }
        trace!("Reached edge of board at {position}");
        positions
    }

    fn check_pawn(&self, position: Position, color: Color, moved: bool) -> Vec<Position> {
        let mut positions = vec![];
        if !moved {
            if let Ok(position) = position
                + (Offset {
                    x: 0,
                    y: 2 * color as i8,
                })
            {
                if self.check_position(position, color, false, false) {
                    positions.push(position);
                };
            };
        };
        if let Ok(position) = position
            + (Offset {
                x: 0,
                y: color as i8,
            })
        {
            if self.check_position(position, color, false, false) {
                positions.push(position);
            };
        };
        if let Ok(position) = position
            + (Offset {
                x: 1,
                y: color as i8,
            })
        {
            if self.check_position(position, color, true, true) {
                positions.push(position);
            };
        };
        if let Ok(position) = position
            + (Offset {
                x: -1,
                y: color as i8,
            })
        {
            if self.check_position(position, color, true, true) {
                positions.push(position);
            };
        };

        positions
    }

    fn check_knight(&self, position: Position, color: Color) -> Vec<Position> {
        let mut positions = vec![];
        let offsets = [
            Offset { x: 2, y: 1 },
            Offset { x: -2, y: 1 },
            Offset { x: -2, y: -1 },
            Offset { x: 2, y: -1 },
            Offset { x: 1, y: 2 },
            Offset { x: -1, y: 2 },
            Offset { x: -1, y: -2 },
            Offset { x: 1, y: -2 },
        ];
        for offset in offsets {
            if let Ok(position) = position + offset {
                if self.check_position(position, color, true, false) {
                    positions.push(position)
                }
            }
        }
        positions
    }

    fn check_king(&self, position: Position, color: Color) -> Vec<Position> {
        let mut positions = vec![];
        let offsets = [
            Offset { x: 1, y: 1 },
            Offset { x: -1, y: 1 },
            Offset { x: -1, y: -1 },
            Offset { x: 1, y: -1 },
            Offset { x: 1, y: 0 },
            Offset { x: -1, y: 0 },
            Offset { x: 0, y: -1 },
            Offset { x: 0, y: 1 },
        ];
        for offset in offsets {
            if let Ok(position) = position + offset {
                if self.check_position(position, color, true, false) {
                    positions.push(position)
                }
            }
        }
        positions
    }

    fn check_position(
        &self,
        position: Position,
        color: Color,
        can_take: bool,
        must_take: bool,
    ) -> bool {
        debug!("Checking {position}");
        let piece = if let Some(piece) = self[position] {
            piece
        } else {
            return !must_take; // Return true for empty square unless must take is true.
        };
        if piece.color == color {
            false
        } else {
            can_take // Return true if piece can take
        }
    }
}

impl Index<Position> for Board {
    type Output = Option<Piece>;

    #[inline(always)]
    fn index(&self, index: Position) -> &Self::Output {
        &self.pieces[(index.y.into(), index.x.into())]
    }
}

impl IndexMut<Position> for Board {
    #[inline(always)]
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.pieces[(index.y.into(), index.x.into())]
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.pieces.rows_iter().enumerate() {
            write!(f, "{}  ", i + 1)?;
            for piece in row {
                write!(
                    f,
                    "{}  ",
                    match piece {
                        Some(piece) => {
                            format!("{piece}")
                        }
                        None => " ".to_string(),
                    }
                )?;
            }
            writeln!(f)?;
            writeln!(f)?;
        }
        write!(f, "   A  B  C  D  E  F  G  H")
    }
}

#[cfg(test)]
mod position_tests {
    use super::*;

    #[test]
    fn test_offset_positive_n() {
        assert_eq!(
            Position { x: 6, y: 6 },
            (Position { x: 6, y: 5 } + Offset { x: 0, y: 1 }).unwrap()
        );
    }

    #[test]
    fn test_offset_positive_ne() {
        assert_eq!(
            Position { x: 6, y: 6 },
            (Position { x: 5, y: 5 } + Offset { x: 1, y: 1 }).unwrap()
        );
    }

    #[test]
    fn test_offset_negative_s() {
        assert_eq!(
            Position { x: 6, y: 5 },
            (Position { x: 6, y: 6 } + Offset { x: 0, y: -1 }).unwrap()
        );
    }

    #[test]
    fn test_offset_negative_sw() {
        assert_eq!(
            Position { x: 5, y: 5 },
            (Position { x: 6, y: 6 } + Offset { x: -1, y: -1 }).unwrap()
        );
    }
}

#[cfg(test)]
mod board_tests {
    use super::*;

    mod move_piece {
        use super::*;

        #[test]
        fn move_queen() {
            let mut board = Board::new();
            board
                .move_piece(Position { x: 3, y: 0 }, Position { x: 5, y: 5 })
                .unwrap();
            assert_eq!(board[Position { x: 3, y: 0 }], None);
            assert_eq!(
                board[Position { x: 5, y: 5 }].unwrap(),
                Piece {
                    color: Color::White,
                    piece_type: PieceType::Queen,
                    moved: true
                }
            )
        }
    }

    mod calculate_possible_moves {
        use super::*;

        #[test]
        fn pawn() {
            let mut board = Board::new();
            board
                .move_piece(Position { x: 5, y: 6 }, Position { x: 5, y: 4 })
                .unwrap();
            board
                .move_piece(Position { x: 4, y: 1 }, Position { x: 4, y: 3 })
                .unwrap();
            let mut result = board
                .calculate_possible_moves(Position { x: 4, y: 3 })
                .unwrap();
            result.sort();
            let mut expected_result = vec![Position { x: 5, y: 4 }, Position { x: 4, y: 4 }];
            expected_result.sort();
            assert_eq!(result, expected_result)
        }

        #[test]
        fn bishop() {
            let mut board = Board::new();
            board
                .move_piece(Position { x: 2, y: 7 }, Position { x: 4, y: 5 })
                .unwrap();
            let mut result = board
                .calculate_possible_moves(Position { x: 4, y: 5 })
                .unwrap();
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
        fn rook() {
            let mut board = Board::new();
            board
                .move_piece(Position { x: 0, y: 0 }, Position { x: 3, y: 4 })
                .unwrap();
            let mut result = board
                .calculate_possible_moves(Position { x: 3, y: 4 })
                .unwrap();
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
        fn queen() {
            let mut board = Board::new();
            board
                .move_piece(Position { x: 3, y: 7 }, Position { x: 1, y: 3 })
                .unwrap();
            let mut result = board
                .calculate_possible_moves(Position { x: 1, y: 3 })
                .unwrap();
            result.sort();
            let mut expected_result = vec![
                Position { x: 0, y: 3 },
                Position { x: 2, y: 3 },
                Position { x: 3, y: 3 },
                Position { x: 4, y: 3 },
                Position { x: 5, y: 3 },
                Position { x: 6, y: 3 },
                Position { x: 7, y: 3 },
                Position { x: 1, y: 1 },
                Position { x: 1, y: 2 },
                Position { x: 1, y: 4 },
                Position { x: 1, y: 5 },
                Position { x: 0, y: 2 },
                Position { x: 2, y: 4 },
                Position { x: 3, y: 5 },
                Position { x: 0, y: 4 },
                Position { x: 2, y: 2 },
                Position { x: 3, y: 1 },
            ];
            expected_result.sort();
            assert_eq!(result, expected_result)
        }
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
                vec![
                    Direction::N,
                    Direction::NE,
                    Direction::E,
                    Direction::SE,
                    Direction::S,
                    Direction::SW,
                    Direction::W,
                    Direction::NW,
                ],
                Color::Black,
            );
            result.sort();
            let mut expected_result = vec![
                Position { x: 0, y: 3 },
                Position { x: 2, y: 3 },
                Position { x: 3, y: 3 },
                Position { x: 4, y: 3 },
                Position { x: 5, y: 3 },
                Position { x: 6, y: 3 },
                Position { x: 7, y: 3 },
                Position { x: 1, y: 1 },
                Position { x: 1, y: 2 },
                Position { x: 1, y: 4 },
                Position { x: 1, y: 5 },
                Position { x: 0, y: 2 },
                Position { x: 2, y: 4 },
                Position { x: 3, y: 5 },
                Position { x: 0, y: 4 },
                Position { x: 2, y: 2 },
                Position { x: 3, y: 1 },
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
                board.check_direction(Position { x: 3, y: 0 }, Direction::S, Color::White),
                vec![]
            );
        }

        #[test]
        fn from_edge_e() {
            let board = Board::new();
            let mut result =
                board.check_direction(Position { x: 0, y: 2 }, Direction::E, Color::White);
            result.sort();
            let mut expected_result = vec![
                Position { x: 1, y: 2 },
                Position { x: 2, y: 2 },
                Position { x: 3, y: 2 },
                Position { x: 4, y: 2 },
                Position { x: 5, y: 2 },
                Position { x: 6, y: 2 },
                Position { x: 7, y: 2 },
            ];
            expected_result.sort();
            assert_eq!(result, expected_result);
        }

        #[test]
        fn from_edge_w() {
            let board = Board::new();
            let mut result =
                board.check_direction(Position { x: 7, y: 4 }, Direction::W, Color::White);
            result.sort();
            let mut expected_result = vec![
                Position { x: 0, y: 4 },
                Position { x: 1, y: 4 },
                Position { x: 2, y: 4 },
                Position { x: 3, y: 4 },
                Position { x: 4, y: 4 },
                Position { x: 5, y: 4 },
                Position { x: 6, y: 4 },
            ];
            expected_result.sort();
            assert_eq!(result, expected_result);
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

    mod check_pawn {
        use super::*;

        #[test]
        fn first_move_white() {
            let board = Board::new();
            let mut result = board.check_pawn(Position { x: 3, y: 3 }, Color::White, false);
            result.sort();
            let mut expected_result = vec![Position { x: 3, y: 4 }, Position { x: 3, y: 5 }];
            expected_result.sort();
            assert_eq!(result, expected_result);
        }

        #[test]
        fn first_move_black() {
            let board = Board::new();
            let mut result = board.check_pawn(Position { x: 6, y: 5 }, Color::Black, false);
            result.sort();
            let mut expected_result = vec![Position { x: 6, y: 4 }, Position { x: 6, y: 3 }];
            expected_result.sort();
            assert_eq!(result, expected_result);
        }

        #[test]
        fn take_two_white() {
            let board = Board::new();
            let mut result = board.check_pawn(Position { x: 6, y: 5 }, Color::White, true);
            result.sort();
            let mut expected_result = vec![Position { x: 5, y: 6 }, Position { x: 7, y: 6 }];
            expected_result.sort();
            assert_eq!(result, expected_result);
        }

        #[test]
        fn take_one_black() {
            let mut board = Board::new();
            board
                .move_piece(Position { x: 2, y: 1 }, Position { x: 2, y: 3 })
                .unwrap();
            let mut result = board.check_pawn(Position { x: 3, y: 4 }, Color::Black, true);
            result.sort();
            let mut expected_result = vec![Position { x: 2, y: 3 }, Position { x: 3, y: 3 }];
            expected_result.sort();
            assert_eq!(result, expected_result);
        }
    }

    mod check_knight {
        use super::*;

        #[test]
        fn no_edge() {
            let board = Board::new();
            let mut result = board.check_knight(Position { x: 3, y: 5 }, Color::White);
            result.sort();
            let mut expected_result = vec![
                Position { x: 2, y: 7 },
                Position { x: 4, y: 7 },
                Position { x: 1, y: 6 },
                Position { x: 5, y: 6 },
                Position { x: 5, y: 4 },
                Position { x: 1, y: 4 },
                Position { x: 2, y: 3 },
                Position { x: 4, y: 3 },
            ];
            expected_result.sort();
            assert_eq!(result, expected_result);
        }

        #[test]
        fn near_edge() {
            let board = Board::new();
            let mut result = board.check_knight(Position { x: 0, y: 5 }, Color::Black);
            result.sort();
            let mut expected_result = vec![Position { x: 2, y: 4 }, Position { x: 1, y: 3 }];
            expected_result.sort();
            assert_eq!(result, expected_result);
        }
    }

    mod check_king {
        use super::*;

        #[test]
        fn near_friendlies() {
            let board = Board::new();
            let mut result = board.check_king(Position { x: 3, y: 5 }, Color::Black);
            result.sort();
            let mut expected_result = vec![
                Position { x: 2, y: 5 },
                Position { x: 2, y: 4 },
                Position { x: 3, y: 4 },
                Position { x: 4, y: 4 },
                Position { x: 4, y: 5 },
            ];
            expected_result.sort();
            assert_eq!(result, expected_result);
        }
    }

    mod check_position {
        use super::*;

        #[test]
        fn must_take_empty() {
            let board = Board::new();
            assert_eq!(
                board.check_position(Position { x: 4, y: 3 }, Color::White, true, true),
                false
            )
        }

        #[test]
        fn must_take_enemy() {
            let board = Board::new();
            assert_eq!(
                board.check_position(Position { x: 0, y: 1 }, Color::Black, true, true),
                true
            )
        }

        #[test]
        fn must_take_friendly() {
            let board = Board::new();
            assert_eq!(
                board.check_position(Position { x: 4, y: 1 }, Color::White, true, true),
                false
            )
        }

        #[test]
        fn cannot_take() {
            let board = Board::new();
            assert_eq!(
                board.check_position(Position { x: 6, y: 1 }, Color::Black, false, false),
                false
            )
        }
    }
}
