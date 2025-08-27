pub mod core;
mod gui;
pub use core::{board::*, cell::*};

pub use gui::{
    Application, ArcLock, Screen, assets, config::*, scale_factor, subscription, theme, update,
    view,
};
