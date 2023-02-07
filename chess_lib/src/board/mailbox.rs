use crate::piece::{Color, Piece, PieceType};
use crate::board::{Position, Offset, Direction};
use crate::error::PieceError;
use array2d::Array2D;
use log::{debug, info, trace, warn};
use std::{
    ops::{Index, IndexMut},
};

use crate::board::layout::DEFAULT_BOARD;



/// Standard 8x8 chess board. Keeps track of positions of pieces.
///
/// Has the capability to check the possible positions a piece could move to. It does not keep track of any game state, and therefore will not account for checks, pins or blocks.
/// Can be indexed with a position, which will return either the piece at that position or None if no piece is present.
///
/// ```
/// use chess_lib::{board::{*, mailbox::*}, piece::*};
///
/// let b = Board::new();
/// assert_eq!(b[Position::new(0, 0).unwrap()], Some(Piece::new(Color::White, PieceType::Rook)));
/// assert_eq!(b[Position::new(0, 2).unwrap()], None);
/// ```
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Board {
    pieces: Array2D<Option<Piece>>,
}

impl Board {
    /// Creates a chess board with a standard layout.
    ///
    /// ```
    /// use chess_lib::{board::{*, mailbox::*}, piece::*};
    ///
    /// let b = Board::new();
    /// assert_eq!(b[Position::new(2, 0).unwrap()], Some(Piece::new(Color::White, PieceType::Bishop)));
    /// assert_eq!(b[Position::new(3, 6).unwrap()], Some(Piece::new(Color::Black, PieceType::Pawn)));
    /// assert_eq!(b[Position::new(0, 2).unwrap()], None);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            pieces: DEFAULT_BOARD.clone(),
        }
    }

    /// Moves piece from `from_position` to `to_position`.
    ///
    /// Does not check if move is possible.
    ///
    /// # Parameters
    /// * `from_position`: The position the piece is currently at.
    /// * `to_position`: The position to move the piece to.
    ///
    /// # Errors
    /// * Returns [`PieceError::NotFound`] error if piece does not exist.
    /// * Returns [`PieceError::Occupied`] error if destination is already occupied.
    pub fn move_piece(
        &mut self,
        from_position: Position,
        to_position: Position,
    ) -> Result<(), PieceError> {
        info!("Moving piece from {from_position} to {to_position}");
        if let Some(piece) = self[to_position] {
            return Err(PieceError::Occupied(to_position, piece.piece_type));
        }

        let Some(mut piece) = self[from_position] else {
            return Err(PieceError::NotFound(
                from_position,
            ));
        };
        piece.moved = true;
        self[from_position] = Some(piece);
        self[to_position] = self[from_position];
        self[from_position] = None;
        Ok(())
    }

    /// Removes piece.
    /// 
    /// # Parameters
    /// * `position`: The position of the piece to remove.
    /// # Errors
    /// * Returns [`PieceError::NotFound`] if piece does not exist.
    /// 
    /// ```
    /// use chess_lib::{board::{*, mailbox::*}, piece::*};
    ///
    /// let mut b = Board::new();
    /// assert_eq!(b[Position::new(3, 0).unwrap()], Some(Piece::new(Color::White, PieceType::Queen)));
    /// b.take_piece(Position::new(3, 0).unwrap());
    /// assert_eq!(b[Position::new(3, 0).unwrap()], None);
    pub fn take_piece(&mut self, position: Position) -> Result<(), PieceError> {
        match self[position] {
            Some(_) => {self[position] = None; Ok(())},
            None => {Err(PieceError::NotFound(position))},
        }
    }

    /// Takes in the position of a piece, returns all possible positions it could move to.
    ///
    /// Order of returned vector is arbitrary, and should not be relied on (if checking against another vector for equality, should be sorted).
    ///
    /// # Parameters
    /// * `position`: The position of the piece to check.
    /// # Errors
    /// * Returns [`PieceError::NotFound`] error if piece does not exist.
    ///
    /// ```
    /// use chess_lib::board::{*, mailbox::*};
    ///
    /// let b = Board::new();
    /// let mut queen_positions = b.check_positions(Position::new(3, 0).unwrap()).unwrap();
    /// queen_positions.sort();
    /// assert_eq!(queen_positions, vec![]);
    ///
    /// let mut pawn_positions = b.check_positions(Position::new(2, 1).unwrap()).unwrap();
    /// pawn_positions.sort();
    /// let mut expected_pawn_positions = vec![Position::new(2, 2).unwrap(), Position::new(2, 3).unwrap()];
    /// expected_pawn_positions.sort();
    /// assert_eq!(pawn_positions, expected_pawn_positions);
    /// ```
    ///
    /// ```
    /// use chess_lib::board::{*, mailbox::*};
    ///
    /// let b = Board::new();
    /// assert!(b.check_positions(Position::new(3, 2).unwrap()).is_err())
    /// ```
    pub fn check_positions(&self, position: Position) -> Result<Vec<Position>, PieceError> {
        use Direction::{E, N, NE, NW, S, SE, SW, W};
        info!("Calculating possible moves for piece at {position}");
        let piece = if let Some(piece) = self[position] {
            debug!("Piece type is {:?}", piece.piece_type);
            piece
        } else {
            warn!("No piece found at {position}");
            return Err(PieceError::NotFound(position));
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
    ///
    /// # Parameters
    /// * `position`: The position to check directions from.
    /// * `directions`: Which directions to check. Order does not matter.
    /// * `color`: Which color the piece being checked is (to determine which pieces can be taken).
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
    ///
    /// # Parameters
    /// * `position`: The position to check the direction from.
    /// * `direction`: Which direction to check. Order does not matter.
    /// * `color`: Which color the piece being checked is (to determine which pieces can be taken).
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
            let Some(piece) = self[position] else {
                positions.push(position);
                continue;
            };
            if piece.color == color {
                trace!("Reached piece of own color at {position}");
                return positions;
            }
            trace!("Reached piece of opposite color at {position}");
            positions.push(position);
            return positions;
        }
        trace!("Reached edge of board at {position}");
        positions
    }

    /// Returns vector of possible positions pawn could move to.
    ///
    /// # Parameters
    /// * `position`: The postition to check movement from.
    /// * `color`: The color that the pawn is (to determine which pieces can be taken).
    /// * `moved`: Whether the pawn has been moved.
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

    /// Returns vector of possible positions knight could move to.
    ///
    /// # Parameters
    /// * `position`: The position to check movement from.
    /// * `color`: The color that the pawn is (to determine which pieces can be taken).
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
                    positions.push(position);
                }
            }
        }
        positions
    }

    /// Returns vector of possible positions king could move to.
    ///
    /// Does NOT check for checks.
    /// # Parameters
    /// * `position`: The position to check movement from.
    /// * `color`: The color that the pawn is (to determine which pieces can be taken).
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
                    positions.push(position);
                }
            }
        }
        positions
    }

    /// Checks whether a position can be moved to.
    ///
    /// # Parameters
    /// * `position`: The position to check movement from.
    /// * `color`: The color that the pawn is (to determine which pieces can be taken).
    /// * `can_take`: Whether other pieces can be taken.
    /// * `must_take`: Whether other pieces must be taken.
    fn check_position(
        &self,
        position: Position,
        color: Color,
        can_take: bool,
        must_take: bool,
    ) -> bool {
        debug!("Checking {position}");
        let  Some(piece) = self[position] else {
            return !must_take; // Return true for empty square unless must take is true.
        };
        if piece.color == color {
            false
        } else {
            can_take // Return true if piece can take
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Index<Position> for Board {
    type Output = Option<Piece>;

    #[inline]
    fn index(&self, index: Position) -> &Self::Output {
        &self.pieces[(index.y.into(), index.x.into())]
    }
}

impl IndexMut<Position> for Board {
    #[inline]
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.pieces[(index.y.into(), index.x.into())]
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

    mod check_positions {
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
            let mut result = board.check_positions(Position { x: 4, y: 3 }).unwrap();
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
            let mut result = board.check_positions(Position { x: 4, y: 5 }).unwrap();
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
            let mut result = board.check_positions(Position { x: 3, y: 4 }).unwrap();
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
            let mut result = board.check_positions(Position { x: 1, y: 3 }).unwrap();
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
