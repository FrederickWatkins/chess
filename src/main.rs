use bevy::prelude::App;

pub mod board;
pub mod piece;

fn main() {
    App::new()
        .add_startup_system(board::setup)
        .add_system(board::show_board)
        .run();
}
