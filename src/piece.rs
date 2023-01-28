use bevy::prelude::{Component, Commands, Bundle};
use ux::u4;

#[derive(Component, Clone, Copy)]
pub enum Color {
    WHITE,
    BLACK,
}

/// Tag all chess pieces
#[derive(Component, Clone, Copy)]
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

pub fn spawn_piece(commands: &mut Commands, piece: Piece, color: Color, position: Position) {
    match piece {
        Piece::PAWN => commands.spawn(PawnBundle::new(color, position)),
        Piece::KNIGHT => commands.spawn(KnightBundle::new(color, position)),
        Piece::BISHOP => commands.spawn(BishopBundle::new(color, position)),
        Piece::ROOK => commands.spawn(RookBundle::new(color, position)),
        Piece::QUEEN => commands.spawn(QueenBundle::new(color, position)),
        Piece::KING => commands.spawn(KingBundle::new(color, position)),
    };
}

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