pub mod core;
mod gui;
pub use core::{board::Board, cell::Cell};

pub use gui::{
    Application, PublicMessage as Message, scale_factor, subscription, theme, update, view,
};
