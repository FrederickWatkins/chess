use bevy::prelude::{Component, Bundle};
use ux::u4;

#[derive(Component)]
pub enum Color {
    WHITE,
    BLACK,
}

/// Tag all chess pieces
#[derive(Component)]
pub enum Piece {
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING
}

#[derive(Component)]
pub struct Position(pub [u4; 2]);

#[derive(Component)]
pub struct Moved(bool);

#[derive(Bundle)]
pub struct PawnBundle {
    color: Color,
    position: Position,
    moved: Moved,
    _piece: Piece,
}

impl PawnBundle {
    pub fn new(color: Color, position: Position) -> Self {
        Self {
            color: color,
            position: position,
            moved: Moved(false),
            _piece: Piece::PAWN,
        }
    }
}

#[derive(Bundle)]
pub struct KnightBundle {
    color: Color,
    position: Position,
    _piece: Piece,
}

impl KnightBundle {
    pub fn new(color: Color, position: Position) -> Self {
        Self {
            color: color,
            position: position,
            _piece: Piece::KNIGHT,
        }
    }
}

#[derive(Bundle)]
pub struct BishopBundle {
    color: Color,
    position: Position,
    _piece: Piece,
}

impl BishopBundle {
    pub fn new(color: Color, position: Position) -> Self {
        Self {
            color: color,
            position: position,
            _piece: Piece::BISHOP,
        }
    }
}

#[derive(Bundle)]
pub struct RookBundle {
    color: Color,
    position: Position,
    moved: Moved,
    _piece: Piece,
}

impl RookBundle {
    pub fn new(color: Color, position: Position) -> Self {
        Self {
            color: color,
            position: position,
            moved: Moved(false),
            _piece: Piece::ROOK,
        }
    }
}

#[derive(Bundle)]
pub struct QueenBundle {
    color: Color,
    position: Position,
    _piece: Piece,
}

impl QueenBundle {
    pub fn new(color: Color, position: Position) -> Self {
        Self {
            color: color,
            position: position,
            _piece: Piece::QUEEN,
        }
    }
}

#[derive(Bundle)]
pub struct KingBundle {
    color: Color,
    position: Position,
    moved: Moved,
    _piece: Piece,
}

impl KingBundle {
    pub fn new(color: Color, position: Position) -> Self {
        Self {
            color: color,
            position: position,
            moved: Moved(false),
            _piece: Piece::KING,
        }
    }
}