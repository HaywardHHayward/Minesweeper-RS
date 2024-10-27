use iced::Theme;
use minesweeper_rs::*;

fn main() -> iced::Result {
    iced::application("Minesweeper", MinesweeperApp::update, MinesweeperApp::view)
        .centered()
        .theme(|_| Theme::Dark)
        .run()
}
