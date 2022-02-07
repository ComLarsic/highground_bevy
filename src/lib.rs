//! The lib for highground
pub mod physics;
pub mod player;

pub mod prelude {
    pub use crate::physics::*;
    pub use crate::player::*;
}
