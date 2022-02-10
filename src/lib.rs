//! The lib for highground
pub mod camera;
pub mod game;
pub mod physics;
pub mod player;

pub mod prelude {
    pub use crate::camera::*;
    pub use crate::game::*;
    pub use crate::physics::*;
    pub use crate::player::*;
}

/// The entrypoint for the game
/// All the logic for the game is build using [`game::GamePlugin`]
/// To be loaded externally from the launcher
#[no_mangle]
pub extern fn start() {
    use bevy::prelude::App;
    // Build and run the app
    let mut app = App::new();
    app.add_plugin(game::GamePlugin);
    app.run();
}