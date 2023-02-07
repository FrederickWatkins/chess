pub mod mailbox;
pub mod layout;

use std::{fmt::Display, ops::Add};

use thiserror::Error;
use crate::piece::{PieceType};

/// Error if a position where no piece is present is passed into a function that requires it.
#[derive(Error, Debug)]

pub enum PieceError {
    #[error("No piece found at {0}.")]
    NotFound(Position),
    #[error("{1:?} already present at {0}")]
    Occupied(Position, PieceType),
}

/// Error if a position is outside of a chess board.
#[derive(Error, Debug, PartialEq)]
#[error("Attempted to create position at {x}, {y}. Position x and y must both be less than 8")]
pub struct PositionOutOfBounds {
    x: isize,
    y: isize,
}

/// Error if an offset is larger than possible for a chess board.
#[derive(Error, Debug)]
#[error("Attempted to create offset of {x}, {y}. Position x and y must both be less than 8 and more than -8")]
pub struct OffsetOutOfBounds {
    x: i8,
    y: i8,
}

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

/// Directions a piece can move. Cardinal and ordinal directions.
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