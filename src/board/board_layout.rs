use crate::piece::{Piece, Color::*, PieceType::*};
use lazy_static::lazy_static;
use array2d::Array2D;

#[rustfmt::skip]
lazy_static! {
    /// Default chess board layout. 0, 0 is A1 etc
    pub static ref DEFAULT_BOARD: Array2D<Option<Piece>> = {
        Array2D::from_row_major(&[
            Some(Piece::new(WHITE, ROOK)),   Some(Piece::new(WHITE, KNIGHT)), Some(Piece::new(WHITE, BISHOP)), Some(Piece::new(WHITE, QUEEN)),  Some(Piece::new(WHITE, KING)),   Some(Piece::new(WHITE, BISHOP)), Some(Piece::new(WHITE, KNIGHT)), Some(Piece::new(WHITE, ROOK)),
        
            Some(Piece::new(WHITE, PAWN)),   Some(Piece::new(WHITE, PAWN)),   Some(Piece::new(WHITE, PAWN)),   Some(Piece::new(WHITE, PAWN)),   Some(Piece::new(WHITE, PAWN)),   Some(Piece::new(WHITE, PAWN)),   Some(Piece::new(WHITE, PAWN)),   Some(Piece::new(WHITE, PAWN)),
        
            None,                  None,                  None,                  None,                  None,                  None,                  None,                  None,
        
            None,                  None,                  None,                  None,                  None,                  None,                  None,                  None,
        
            None,                  None,                  None,                  None,                  None,                  None,                  None,                  None,
        
            None,                  None,                  None,                  None,                  None,                  None,                  None,                  None,
        
            Some(Piece::new(BLACK, PAWN)),   Some(Piece::new(BLACK, PAWN)),   Some(Piece::new(BLACK, PAWN)),   Some(Piece::new(BLACK, PAWN)),   Some(Piece::new(BLACK, PAWN)),   Some(Piece::new(BLACK, PAWN)),   Some(Piece::new(BLACK, PAWN)),   Some(Piece::new(BLACK, PAWN)),
        
            Some(Piece::new(BLACK, ROOK)),   Some(Piece::new(BLACK, KNIGHT)), Some(Piece::new(BLACK, BISHOP)), Some(Piece::new(BLACK, QUEEN)),  Some(Piece::new(BLACK, KING)),   Some(Piece::new(BLACK, BISHOP)), Some(Piece::new(BLACK, KNIGHT)), Some(Piece::new(BLACK, ROOK)),
        ], 8, 8).unwrap()
    };
}