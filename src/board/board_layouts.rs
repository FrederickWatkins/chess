use crate::piece::{Color, Color::*, Piece, Piece::*};
use lazy_static::lazy_static;
use array2d::Array2D;

#[rustfmt::skip]
lazy_static! {
    /// Default chess board layout. 0, 0 is A1 etc
    pub static ref DEFAULT_BOARD: Array2D<Option<(Color, Piece)>> = {
        Array2D::from_row_major(&[
            Some((WHITE, ROOK)),   Some((WHITE, KNIGHT)), Some((WHITE, BISHOP)), Some((WHITE, QUEEN)),  Some((WHITE, KING)),   Some((WHITE, BISHOP)), Some((WHITE, KNIGHT)), Some((WHITE, ROOK)),
        
            Some((WHITE, PAWN)),   Some((WHITE, PAWN)),   Some((WHITE, PAWN)),   Some((WHITE, PAWN)),   Some((WHITE, PAWN)),   Some((WHITE, PAWN)),   Some((WHITE, PAWN)),   Some((WHITE, PAWN)),
        
            None,                  None,                  None,                  None,                  None,                  None,                  None,                  None,
        
            None,                  None,                  None,                  None,                  None,                  None,                  None,                  None,
        
            None,                  None,                  None,                  None,                  None,                  None,                  None,                  None,
        
            None,                  None,                  None,                  None,                  None,                  None,                  None,                  None,
        
            Some((BLACK, PAWN)),   Some((BLACK, PAWN)),   Some((BLACK, PAWN)),   Some((BLACK, PAWN)),   Some((BLACK, PAWN)),   Some((BLACK, PAWN)),   Some((BLACK, PAWN)),   Some((BLACK, PAWN)),
        
            Some((BLACK, ROOK)),   Some((BLACK, KNIGHT)), Some((BLACK, BISHOP)), Some((BLACK, QUEEN)),  Some((BLACK, KING)),   Some((BLACK, BISHOP)), Some((BLACK, KNIGHT)), Some((BLACK, ROOK)),
        ], 8, 8).unwrap()
    };
}