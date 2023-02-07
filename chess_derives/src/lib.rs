extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ExecuteMove)]
pub fn derive_execute_move(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl ExecuteMove for #ident {
            fn execute_move(&self, chess_move: ChessMove) -> Result<(), PieceError> {
                match chess_move {
                    Move(movement) => {self.move_piece(movement.from_position, movement.to_position)?;}
                    MoveWithTake(movement, take) => {
                        self.take_piece(take.position)?;
                        self.move_piece(movement.from_position, movement.to_position)?;
                    }
                    Castle(movement_1, movement_2) => {
                        self.move_piece(movement_1.from_position, movement_1.to_position)?;
                        self.move_piece(movement_2.from_position, movement_2.to_position)?;
                    }
                    Promote(movement, promotion) => {
                        self.move_piece(movement.from_position, movement.to_position)?;
                        self.promote_piece(promotion.position, promotion.piece_type)?;
                    }
                }
                return Ok(());
            }
        }
    };
    output.into()
}
