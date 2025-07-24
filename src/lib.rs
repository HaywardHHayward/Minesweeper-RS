pub mod core;
mod gui;
pub use core::{board::Board, cell::Cell};

pub use gui::{Application, PublicMessage as Message, subscription, theme, update, view};
