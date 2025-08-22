pub mod core;
mod gui;
pub use core::{board::Board, cell::Cell};

pub use gui::{Application, scale_factor, subscription, theme, update, view};
pub(crate) use gui::{RcCell, Screen, config::Config};
