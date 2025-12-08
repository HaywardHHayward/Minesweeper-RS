#![deny(missing_debug_implementations)]
pub mod cli;
pub mod core;
mod gui;
pub use core::{board::*, cell::*};

pub use gui::{
    Application, ArcLock, Screen, assets,
    config::{Config, GameTheme, MenuButtonStyle, MenuTheme},
    scale_factor, subscription, theme, update, view,
};
