use std::fmt::Display;

/// Chess piece colors.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Color {
    White = 1,
    Black = -1,
}

/// Piece types.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

/// Chess piece.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
    pub moved: bool,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.piece_type {
                PieceType::Pawn => "P",
                PieceType::Knight => "N",
                PieceType::Bishop => "B",
                PieceType::Rook => "R",
                PieceType::Queen => "Q",
                PieceType::King => "K",
            }
        )
    }
}

/// Creates new chess piece.
///
/// Chess piece is initialized with moved = false.
impl Piece {
    #[must_use]
    pub fn new(color: Color, piece_type: PieceType) -> Self {
        Self {
            color,
            piece_type,
            moved: false,
        }
    }
}
