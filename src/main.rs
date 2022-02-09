//! A story-driven 2d platformer with rpg-elements, inspired by super paper mario
use bevy::prelude::*;
use libhighground::prelude::*;

/// The entrypoint is the most basic possible.
/// All the setup is done in the [`GamePlugin`]
fn main() {
    let mut app = App::new();
    app.add_plugin(GamePlugin);
    app.run();
}
