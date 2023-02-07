use thiserror::Error;
use crate::{piece::PieceType, board::Position};
/// Error if a position where no piece is present is passed into a function that requires it.
#[derive(Error, Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum PieceError {
    #[error("No piece found at {0}.")]
    NotFound(Position),
    #[error("{1:?} already present at {0}")]
    Occupied(Position, PieceType),
}

/// Error if a position is outside of a chess board.
#[derive(Error, Debug, PartialEq)]
#[error("Attempted to create position at {0}, {1}. Position x and y must both be less than 8")]
pub struct PositionOutOfBounds (pub isize, pub isize);

/// Error if an offset is larger than possible for a chess board.
#[derive(Error, Debug)]
#[error("Attempted to create offset of {0}, {1}. Position x and y must both be less than 8 and more than -8")]
pub struct OffsetOutOfBounds (pub i8, pub i8);