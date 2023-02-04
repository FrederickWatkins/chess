use crate::piece::*;
use array2d::Array2D;
use log::{debug, info, trace, warn};
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Index, IndexMut},
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
            Err(PositionOutOfBounds {x: x.into(), y: y.into()})
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
        let (new_x, new_y) = unsafe {(i8::try_from(self.x).unwrap_unchecked(), i8::try_from(self.y).unwrap_unchecked())};  // This is okay since x and y must always be less than 8
        Self::new(
            match new_x.try_into() {
                Ok(x) => x,
                Err(_) => {return Err(PositionOutOfBounds {x: new_x.into(), y: new_y.into()});} ,
            },
            match new_y.try_into() {
                Ok(y) => y,
                Err(_) => {return Err(PositionOutOfBounds {x: new_x.into(), y: new_y.into()});} ,
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
    pub fn move_piece(&mut self, from_position: Position, to_position: Position) {
        info!("Moving piece from {from_position} to {to_position}");
        self[to_position] = None;
        let mut piece = self[from_position].unwrap();
        piece.moved = true;
        self[from_position] = Some(piece);
        self[to_position] = self[from_position];
        self[from_position] = None;
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
            PieceType::Pawn => todo!(),
            PieceType::Knight => todo!(),
            PieceType::Bishop => self.check_directions(position, vec![NE, SE, SW, NW], piece.color),
            PieceType::Rook => self.check_directions(position, vec![N, E, S, W], piece.color),
            PieceType::Queen => {
                self.check_directions(position, vec![N, NE, E, SE, S, SW, W, NW], piece.color)
            }
            PieceType::King => todo!(),
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
        let offset = unsafe {match direction {
            Direction::N => Offset::new(0, 1),
            Direction::NE => Offset::new(1, 1),
            Direction::E => Offset::new(1, 0),
            Direction::SE => Offset::new(1, -1),
            Direction::S => Offset::new(0, -1),
            Direction::SW => Offset::new(-1, -1),
            Direction::W => Offset::new(-1, 0),
            Direction::NW => Offset::new(-1, 1),
        }.unwrap_unchecked()}; // This is okay because all match arms create valid offsets
        while 0 < position.x && position.x < 7 && 0 < position.y && position.y < 7 {
            position = (position + offset).unwrap();
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

    fn check_offset(
        &self,
        mut position: Position,
        color: Color,
        offset: Offset,
        can_take: bool,
    ) -> bool {
        debug!("Checking offset {offset} from {position}");
        position = (position + offset).unwrap();
        let piece = if let Some(piece) = self[position] {
            piece
        } else {
            return true;
        };
        if piece.color == color {
            false
        } else if can_take == true {
            true
        } else {
            false
        }
    }
}

impl Index<Position> for Board {
    type Output = Option<Piece>;

    #[inline(always)]
    fn index(&self, index: Position) -> &Self::Output {
        &self.pieces[(index.y.try_into().unwrap(), index.x.try_into().unwrap())]
    }
}

impl IndexMut<Position> for Board {
    #[inline(always)]
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.pieces[(index.y.try_into().unwrap(), index.x.try_into().unwrap())]
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
        assert_eq!(Position::new(6, 6), Position::new(6, 5) + Offset::new(0, 1));
    }

    #[test]
    fn test_offset_positive_ne() {
        assert_eq!(Position::new(6, 6), Position::new(5, 5) + Offset::new(1, 1));
    }

    #[test]
    fn test_offset_negative_s() {
        assert_eq!(
            Position::new(6, 5),
            Position::new(6, 6) + Offset::new(0, -1)
        );
    }

    #[test]
    fn test_offset_negative_sw() {
        assert_eq!(
            Position::new(5, 5),
            Position::new(6, 6) + Offset::new(-1, -1)
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
            board.move_piece(Position::new(3, 0), Position::new(5, 5));
            assert_eq!(board[Position::new(3, 0)], None);
            assert_eq!(
                board[Position::new(5, 5)].unwrap(),
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
        fn bishop() {
            let mut board = Board::new();
            board.move_piece(Position::new(2, 7), Position::new(4, 5));
            let mut result = board.calculate_possible_moves(Position::new(4, 5)).unwrap();
            result.sort();
            let mut expected_result = vec![
                Position::new(0, 1),
                Position::new(1, 2),
                Position::new(2, 3),
                Position::new(3, 4),
                Position::new(7, 2),
                Position::new(6, 3),
                Position::new(5, 4),
            ];
            expected_result.sort();
            assert_eq!(result, expected_result)
        }

        #[test]
        fn rook() {
            let mut board = Board::new();
            board.move_piece(Position::new(0, 0), Position::new(3, 4));
            let mut result = board.calculate_possible_moves(Position::new(3, 4)).unwrap();
            result.sort();
            let mut expected_result = vec![
                Position::new(0, 4),
                Position::new(1, 4),
                Position::new(2, 4),
                Position::new(4, 4),
                Position::new(5, 4),
                Position::new(6, 4),
                Position::new(7, 4),
                Position::new(3, 2),
                Position::new(3, 3),
                Position::new(3, 5),
                Position::new(3, 6),
            ];
            expected_result.sort();
            assert_eq!(result, expected_result)
        }

        #[test]
        fn queen() {
            let mut board = Board::new();
            board.move_piece(Position::new(3, 7), Position::new(1, 3));
            let mut result = board.calculate_possible_moves(Position::new(1, 3)).unwrap();
            result.sort();
            let mut expected_result = vec![
                Position::new(0, 3),
                Position::new(2, 3),
                Position::new(3, 3),
                Position::new(4, 3),
                Position::new(5, 3),
                Position::new(6, 3),
                Position::new(7, 3),
                Position::new(1, 1),
                Position::new(1, 2),
                Position::new(1, 4),
                Position::new(1, 5),
                Position::new(0, 2),
                Position::new(2, 4),
                Position::new(3, 5),
                Position::new(0, 4),
                Position::new(2, 2),
                Position::new(3, 1),
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
                Position::new(3, 4),
                vec![Direction::N, Direction::E, Direction::S, Direction::W],
                Color::White,
            );
            result.sort();
            let mut expected_result = vec![
                Position::new(0, 4),
                Position::new(1, 4),
                Position::new(2, 4),
                Position::new(4, 4),
                Position::new(5, 4),
                Position::new(6, 4),
                Position::new(7, 4),
                Position::new(3, 2),
                Position::new(3, 3),
                Position::new(3, 5),
                Position::new(3, 6),
            ];
            expected_result.sort();
            assert_eq!(result, expected_result)
        }

        #[test]
        fn diagonal_directions() {
            let board = Board::new();
            let mut result = board.check_directions(
                Position::new(4, 5),
                vec![Direction::NE, Direction::SE, Direction::SW, Direction::NW],
                Color::Black,
            );
            result.sort();
            let mut expected_result = vec![
                Position::new(0, 1),
                Position::new(1, 2),
                Position::new(2, 3),
                Position::new(3, 4),
                Position::new(7, 2),
                Position::new(6, 3),
                Position::new(5, 4),
            ];
            expected_result.sort();
            assert_eq!(result, expected_result)
        }

        #[test]
        fn all_directions() {
            let board = Board::new();
            let mut result = board.check_directions(
                Position::new(1, 3),
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
                Position::new(0, 3),
                Position::new(2, 3),
                Position::new(3, 3),
                Position::new(4, 3),
                Position::new(5, 3),
                Position::new(6, 3),
                Position::new(7, 3),
                Position::new(1, 1),
                Position::new(1, 2),
                Position::new(1, 4),
                Position::new(1, 5),
                Position::new(0, 2),
                Position::new(2, 4),
                Position::new(3, 5),
                Position::new(0, 4),
                Position::new(2, 2),
                Position::new(3, 1),
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
                board.check_direction(Position::new(4, 0), Direction::N, Color::White),
                vec![]
            );
        }

        #[test]
        fn no_move_w() {
            let board = Board::new();
            assert_eq!(
                board.check_direction(Position::new(5, 1), Direction::W, Color::White),
                vec![]
            );
        }

        #[test]
        fn edge_board_e() {
            let board = Board::new();
            assert_eq!(
                board.check_direction(Position::new(6, 5), Direction::E, Color::White),
                vec![Position::new(7, 5)]
            );
        }

        #[test]
        fn edge_board_s() {
            let board = Board::new();
            assert_eq!(
                board.check_direction(Position::new(3, 7), Direction::E, Color::White),
                vec![]
            );
        }

        #[test]
        fn take_piece_ne() {
            let board = Board::new();
            let mut result =
                board.check_direction(Position::new(2, 2), Direction::NE, Color::White);
            result.sort();
            let mut expected_result = vec![
                Position::new(3, 3),
                Position::new(4, 4),
                Position::new(5, 5),
                Position::new(6, 6),
            ];
            expected_result.sort();
            assert_eq!(result, expected_result);
        }

        #[test]
        fn take_piece_sw() {
            let board = Board::new();
            let mut result =
                board.check_direction(Position::new(4, 3), Direction::SW, Color::Black);
            result.sort();
            let mut expected_result = vec![Position::new(3, 2), Position::new(2, 1)];
            expected_result.sort();
            assert_eq!(result, expected_result);
        }
    }
}
