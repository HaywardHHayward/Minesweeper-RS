pub mod core;
mod gui;
pub use core::{board::Board, cell::Cell};

pub use gui::{
    Application, ArcLock, Screen, config::Config, scale_factor, subscription, theme, update, view,
};
