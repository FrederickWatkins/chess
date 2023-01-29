#[derive(Eq, PartialEq, Clone, Copy)]
pub enum Color {
    WHITE,
    BLACK,
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum PieceType {
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING,
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
