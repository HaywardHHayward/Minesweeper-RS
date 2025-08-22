use crate::{Config, RcCell, Screen};

#[derive(Debug)]
pub enum Message {}

#[derive(Debug)]
pub struct MainMenu {
    config: RcCell<Config>,
}

impl Screen for MainMenu {}
