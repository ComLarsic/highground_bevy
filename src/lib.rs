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
