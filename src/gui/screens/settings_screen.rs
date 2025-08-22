use crate::{Config, RcCell, Screen};

#[derive(Debug)]
pub enum Message {}

#[derive(Debug)]
pub struct SettingsScreen {
    config: RcCell<Config>,
}

impl Screen for SettingsScreen {}
