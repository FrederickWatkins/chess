#[derive(Eq, PartialEq, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
}

impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Self {
        Self { color, piece_type }
    }
}
