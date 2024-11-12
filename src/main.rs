use iced::{Font, Theme};
use minesweeper_rs::*;

fn main() -> iced::Result {
    iced::application("Minesweeper", MinesweeperApp::update, MinesweeperApp::view)
        .subscription(MinesweeperApp::subscription)
        .centered()
        .theme(|_| Theme::Light)
        .default_font(Font::MONOSPACE)
        .run()
}
