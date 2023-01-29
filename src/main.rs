use bevy::prelude::{App, IntoSystemDescriptor};

pub mod board;
pub mod piece;

fn main() {
    App::new()
        .add_startup_system(board::setup)
        .add_system(board::show_board)
        .add_system(board::get_input.after(board::show_board))
        .run();
}
