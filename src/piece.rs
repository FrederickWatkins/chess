use bevy::prelude::{Component, Bundle};
use ux::u4;

#[derive(Component)]
pub enum Color {
    WHITE,
    BLACK,
}

/// Tag all chess pieces
#[derive(Component)]
struct Piece;

#[derive(Component)]
pub struct Position([u4; 2]);

#[derive(Component)]
struct Moved(bool);

#[derive(Component)]
struct Pawn;

#[derive(Component)]
struct Knight;

#[derive(Component)]
struct Bishop;

#[derive(Component)]
struct Rook;

#[derive(Component)]
struct Queen;

#[derive(Component)]
struct King;

#[derive(Bundle)]
pub struct PawnBundle {
    color: Color,
    position: Position,
    moved: Moved,
    _piece: Piece,
    _pawn: Pawn,
}

impl PawnBundle {
    pub fn new(color: Color, position: Position) -> Self {
        Self {
            color: color,
            position: position,
            moved: Moved(false),
            _piece: Piece,
            _pawn: Pawn,
        }
    }
}

#[derive(Bundle)]
pub struct KnightBundle {
    color: Color,
    position: Position,
    _piece: Piece,
    _knight: Knight,
}

impl KnightBundle {
    pub fn new(color: Color, position: Position) -> Self {
        Self {
            color: color,
            position: position,
            _piece: Piece,
            _knight: Knight,
        }
    }
}

#[derive(Bundle)]
pub struct BishopBundle {
    color: Color,
    position: Position,
    _piece: Piece,
    _bishop: Bishop,
}

impl BishopBundle {
    pub fn new(color: Color, position: Position) -> Self {
        Self {
            color: color,
            position: position,
            _piece: Piece,
            _bishop: Bishop,
        }
    }
}

#[derive(Bundle)]
pub struct RookBundle {
    color: Color,
    position: Position,
    moved: Moved,
    _piece: Piece,
    _rook: Rook,
}

impl RookBundle {
    pub fn new(color: Color, position: Position) -> Self {
        Self {
            color: color,
            position: position,
            moved: Moved(false),
            _piece: Piece,
            _rook: Rook,
        }
    }
}

#[derive(Bundle)]
pub struct QueenBundle {
    color: Color,
    position: Position,
    _piece: Piece,
    _queen: Queen,
}

impl QueenBundle {
    pub fn new(color: Color, position: Position) -> Self {
        Self {
            color: color,
            position: position,
            _piece: Piece,
            _queen: Queen,
        }
    }
}

#[derive(Bundle)]
pub struct KingBundle {
    color: Color,
    position: Position,
    moved: Moved,
    _piece: Piece,
    _king: King,
}

impl KingBundle {
    pub fn new(color: Color, position: Position) -> Self {
        Self {
            color: color,
            position: position,
            moved: Moved(false),
            _piece: Piece,
            _king: King,
        }
    }
}