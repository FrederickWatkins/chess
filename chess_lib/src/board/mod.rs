pub mod layout;
pub mod mailbox;


use crate::{error::{OffsetOutOfBounds, PieceError, PositionOutOfBounds}, piece::{PieceType}};
use std::{collections::HashSet, fmt::Display, ops::Add};
/// Position on chess board.
///
/// (0, 0) is A1, (7, 7) is H8 etc.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Position {
    x: u8,
    y: u8,
}

impl Position {
    /// Creates a new position (x, y).
    ///
    /// # Parameters
    /// * `x`: The horizontal coordinate
    /// * `y`: The vertical coordinate
    ///
    /// # Errors
    /// * Will return [`PositionOutOfBounds`] error if x and y are not both less than 8.
    ///
    /// ```
    /// use chess_lib::board::Position;
    ///
    /// assert!(Position::new(3, 4).is_ok());
    /// assert!(Position::new(8, 2).is_err());
    /// assert!(Position::new(4, 8).is_err());
    /// ```
    pub fn new(x: u8, y: u8) -> Result<Self, PositionOutOfBounds> {
        if x < 8 && y < 8 {
            Ok(Self { x, y })
        } else {
            Err(PositionOutOfBounds(x.into(), y.into()))
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// Offset to a position on a chess board. Can be added to position.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Offset {
    x: i8,
    y: i8,
}

impl Offset {
    /// Creates a new offset (x, y).
    ///
    /// # Parameters
    /// * `x`: The horizontal component
    /// * `y`: The vertical component
    /// # Errors
    /// * Will return [`OffsetOutOfBounds`] error if x and y are not both between -8 and 8.
    ///
    /// ```
    /// use chess_lib::board::Offset;
    ///
    /// assert!(Offset::new(-2, 4).is_ok());
    /// assert!(Offset::new(8, -2).is_err());
    /// assert!(Offset::new(4, -8).is_err());
    /// ```
    pub fn new(x: i8, y: i8) -> Result<Self, OffsetOutOfBounds> {
        if -8 < x && x < 8 && -8 < y && y < 8 {
            Ok(Self { x, y })
        } else {
            Err(OffsetOutOfBounds(x, y))
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
                    return Err(PositionOutOfBounds(new_x.into(), new_y.into()));
                }
            },
            match new_y.try_into() {
                Ok(y) => y,
                Err(_) => {
                    return Err(PositionOutOfBounds(new_x.into(), new_y.into()));
                }
            },
        )
    }
}

/// Directions a piece can move. Cardinal and ordinal directions.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

pub mod action {
    use super::Position;
    use crate::piece::PieceType;
    pub struct Move {pub from_position: Position, pub to_position: Position}
    pub struct Take {pub position: Position}
    pub struct Promote {pub position: Position, pub piece_type: PieceType}
}

pub enum ChessMove {
    Move(action::Move),
    MoveWithTake(action::Move, action::Take),
    Castle(action::Move, action::Move),
    Promote(action::Move, action::Promote)
}

pub trait ExecuteMove: MovePiece + TakePiece + PromotePiece {
    /// Execute a chess move on the board.
    /// 
    /// Will not check that the move is legal.
    /// # Parameters
    /// * `chess_move`: The move to execute.
    /// # Errors
    /// * Returns [`PieceError::NotFound`] if move attempts to move, take or promote a piece that does not exist.
    /// * Returns [`PieceError::Occupied`] if move attempts to move piece to a square that is already occupied.
    fn execute_move(&self, chess_move: ChessMove) -> Result<(), PieceError>;
}



pub trait MovePiece {
    /// Move a piece on the board.
    /// 
    /// Will not check that move is legal.
    /// # Parameters
    /// * `from_position`: The position the piece is currently at.
    /// * `to_position`: The position to move the piece to.
    /// # Errors
    /// * Returns [`PieceError::NotFound`] if there is no piece at `from_position`.
    /// * Returns [`PieceError::Occupied`] if there is already a piece at `to_position`.
    fn move_piece(&self, from_position: Position, to_position: Position) -> Result<(), PieceError>;
}

pub trait TakePiece {
    /// Take a piece on the board.
    /// 
    /// # Parameters
    /// * `position`: The position of the piece.
    /// # Errors
    /// * Returns [`PieceError::NotFound`] if there is no piece at `position`.
    fn take_piece(&self, position: Position) -> Result<(), PieceError>;
}

pub trait PromotePiece {
    /// Promote a piece on the board.
    /// 
    /// Does not check that promotion is legal.
    /// # Parameters
    /// * `position`: The position of the piece.
    /// # Errors
    /// * Returns [`PieceError::NotFound`] if there is no piece at `position`.
    fn promote_piece(&self, position: Position, piece_type: PieceType) -> Result<(), PieceError>;
}

pub trait PseudoLegalMoves {
    /// Generate pseudo legal moves for piece at `position`.
    /// 
    /// # Parameters
    /// * `position`: The position of the piece.
    /// # Errors
    /// * Returns [`PieceError::NotFound`] if there is no piece at `position`.
    fn pseudo_legal_moves(&self, position: Position) -> Result<HashSet<ChessMove>, PieceError>;
}

pub trait LegalMoves {
    /// Generate legal moves for piece at `position`.
    /// 
    /// # Parameters
    /// * `position`: The position of the piece.
    /// # Errors
    /// * Returns [`PieceError::NotFound`] if there is no piece at `position`.
    fn legal_moves(&self, position: Position) -> Result<HashSet<ChessMove>, PieceError>;
}