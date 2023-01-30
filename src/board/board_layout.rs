use crate::piece::{Piece, Color::*, PieceType::*};
use lazy_static::lazy_static;
use array2d::Array2D;

#[rustfmt::skip]
lazy_static! {
    /// Default chess board layout. 0, 0 is A1 etc
    pub static ref DEFAULT_BOARD: Array2D<Option<Piece>> = {
        Array2D::from_row_major(&[
            Some(Piece::new(White, Rook)),   Some(Piece::new(White, Knight)), Some(Piece::new(White, Bishop)), Some(Piece::new(White, Queen)),  Some(Piece::new(White, King)),   Some(Piece::new(White, Bishop)), Some(Piece::new(White, Knight)), Some(Piece::new(White, Rook)),
        
            Some(Piece::new(White, Pawn)),   Some(Piece::new(White, Pawn)),   Some(Piece::new(White, Pawn)),   Some(Piece::new(White, Pawn)),   Some(Piece::new(White, Pawn)),   Some(Piece::new(White, Pawn)),   Some(Piece::new(White, Pawn)),   Some(Piece::new(White, Pawn)),
        
            None,                  None,                  None,                  None,                  None,                  None,                  None,                  None,
        
            None,                  None,                  None,                  None,                  None,                  None,                  None,                  None,
        
            None,                  None,                  None,                  None,                  None,                  None,                  None,                  None,
        
            None,                  None,                  None,                  None,                  None,                  None,                  None,                  None,
        
            Some(Piece::new(Black, Pawn)),   Some(Piece::new(Black, Pawn)),   Some(Piece::new(Black, Pawn)),   Some(Piece::new(Black, Pawn)),   Some(Piece::new(Black, Pawn)),   Some(Piece::new(Black, Pawn)),   Some(Piece::new(Black, Pawn)),   Some(Piece::new(Black, Pawn)),
        
            Some(Piece::new(Black, Rook)),   Some(Piece::new(Black, Knight)), Some(Piece::new(Black, Bishop)), Some(Piece::new(Black, Queen)),  Some(Piece::new(Black, King)),   Some(Piece::new(Black, Bishop)), Some(Piece::new(Black, Knight)), Some(Piece::new(Black, Rook)),
        ], 8, 8).unwrap()
    };
}